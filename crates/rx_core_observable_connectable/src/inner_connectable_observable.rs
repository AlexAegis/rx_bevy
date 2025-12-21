use std::sync::{Arc, Mutex};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, SubjectLike, Subscriber, SubscriptionLike,
	SubscriptionWithTeardown, TeardownCollection, TeardownCollectionExtension, UpgradeableObserver,
};

use crate::observable::{Connectable, ConnectableOptions, ConnectionHandle};

pub(crate) struct ConnectionOptions {
	disconnect_when_ref_count_zero: bool,
}

pub(crate) struct ConnectionState<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	downstream_subscriber_count: usize,

	connection_options: ConnectionOptions,

	connection: Option<ConnectionHandle<Subscription>>,
}

impl ConnectionOptions {
	pub(crate) fn from_connectable_options<Connector>(value: &ConnectableOptions<Connector>) -> Self
	where
		Connector: 'static + Clone + SubjectLike,
	{
		Self {
			disconnect_when_ref_count_zero: value.disconnect_when_ref_count_zero,
		}
	}
}

impl<Subscription> ConnectionState<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	pub(crate) fn new(connection_options: ConnectionOptions) -> Self {
		Self {
			connection_options,
			downstream_subscriber_count: 0,
			connection: None,
		}
	}

	pub(crate) fn increment_subscriber_count(&mut self) {
		self.downstream_subscriber_count = self.downstream_subscriber_count.saturating_add(1);
	}

	/// Returns if this should cause a disconnect or not
	pub(crate) fn decrement_subscriber_count(&mut self) -> bool {
		self.downstream_subscriber_count = self.downstream_subscriber_count.saturating_sub(1);
		self.connection_options.disconnect_when_ref_count_zero
			&& self.downstream_subscriber_count == 0
	}

	pub(crate) fn disconnect(&mut self) -> bool {
		if let Some(mut connection) = self.connection.take()
			&& !connection.is_closed()
		{
			connection.unsubscribe();
			true
		} else {
			false
		}
	}

	pub(crate) fn is_connected(&self) -> bool {
		if let Some(connection) = self.connection.as_ref() {
			!connection.is_closed()
		} else {
			false
		}
	}

	pub(crate) fn register_connection(
		&mut self,
		connection: Subscription,
	) -> ConnectionHandle<Subscription> {
		self.disconnect();
		let handle = ConnectionHandle::new(connection);
		self.connection = Some(handle.clone());
		handle
	}

	pub(crate) fn get_connection(&self) -> Option<ConnectionHandle<Subscription>> {
		self.connection.clone()
	}

	pub(crate) fn get_active_connection(&mut self) -> Option<ConnectionHandle<Subscription>> {
		self.connection
			.as_ref()
			.filter(|connection| !connection.is_closed())
			.cloned()
	}

	pub(crate) fn is_connection_closed(&self) -> bool {
		self.connection
			.as_ref()
			.map(|connection| connection.is_closed())
			.unwrap_or(true)
	}
}

#[derive(RxObservable)]
#[rx_out(Connector::Out)]
#[rx_out_error(Connector::OutError)]
pub struct ConnectorState<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Arc<Mutex<Option<Connector>>>,

	// connection: Option<
	// 	ConnectionHandle<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>,
	// >,
	connection_state: Arc<
		Mutex<ConnectionState<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>>,
	>,

	// downstream_subscriber_count: usize,
	//
	// connection: Option<
	// 	ConnectionHandle<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>,
	// >,
	options: ConnectableOptions<Connector>,
}

impl<Source, Connector> ConnectorState<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	pub(crate) fn new(
		source: Source,
		connection: Arc<
			Mutex<
				ConnectionState<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>,
			>,
		>,
		options: ConnectableOptions<Connector>,
	) -> Self {
		Self {
			source,
			connector: Arc::new(Mutex::new(None)),
			connection_state: connection,
			options,
		}
	}

	fn get_connector(&mut self) -> Connector {
		let mut connector = self.connector.lock_ignore_poison();
		connector
			.get_or_insert_with(|| (self.options.connector_creator)())
			.clone()
	}

	fn get_active_connector(&mut self) -> Connector {
		{
			let mut connector = self.connector.lock_ignore_poison();

			// Remove the connector if it's closed, and only when it's closed
			if connector
				.as_ref()
				.map(|connector| connector.is_closed())
				.unwrap_or(false)
			{
				connector.take();
			}
		}

		self.get_connector()
	}

	fn create_connection(
		&mut self,
	) -> Source::Subscription<<Connector as UpgradeableObserver>::Upgraded> {
		let connector = self.get_active_connector().clone();

		let mut connection = self.source.subscribe(connector.clone());

		let connector_arc = self.connector.clone();
		if self.options.reset_connector_on_disconnect {
			connection.add_fn(move || {
				// Simply drop the connector from behind the mutex, so that
				// new connections will be forced to create a new connector.
				connector_arc.lock_ignore_poison().take();
				// Attempting to unsubscribe the connector here will lead to a deadlock!
			});
		}

		connection
	}

	pub(crate) fn register_connection(
		&mut self,
		connection: Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>,
	) -> ConnectionHandle<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>> {
		self.connection_state
			.lock_clear_poison()
			.register_connection(connection)
	}
}

impl<Source, Connector> SubscriptionLike for ConnectorState<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.connection_state
			.lock_ignore_poison()
			.is_connection_closed()
	}

	fn unsubscribe(&mut self) {
		let mut connector = self.connector.lock_ignore_poison();
		if let Some(connector) = connector.as_mut() {
			println!("UNSUB STATE");
			connector.unsubscribe();
		}
	}
}

impl<Source, Connector> Observable for ConnectorState<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	type Subscription<Destination>
		= Connector::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let mut connector = self.get_active_connector();
		connector.subscribe(observer)
	}
}

impl<Source, Connector> Connectable for ConnectorState<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	type ConnectionSubscription =
		Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>;

	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription> {
		let active_connection = self
			.connection_state
			.lock_ignore_poison()
			.get_active_connection();

		active_connection.unwrap_or_else(move || {
			let new_connection = self.create_connection();
			self.register_connection(new_connection)
		})
	}

	fn disconnect(&mut self) -> bool {
		self.connection_state.lock_ignore_poison().disconnect()
	}

	fn is_connected(&self) -> bool {
		self.connection_state.lock_ignore_poison().is_connected()
	}

	fn reset(&mut self) {
		self.disconnect();
		self.connector.lock_ignore_poison().take();
		self.connection_state
			.lock_ignore_poison()
			.downstream_subscriber_count = 0;
	}
}
