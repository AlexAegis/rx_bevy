use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, SubjectLike, Subscriber, SubscriptionLike, Teardown, TeardownCollection,
	TeardownCollectionExtension, UpgradeableObserver,
};

use crate::observable::{Connectable, ConnectableOptions, ConnectionHandle};

#[derive(RxObservable)]
#[rx_out(Connector::Out)]
#[rx_out_error(Connector::OutError)]
pub struct InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Option<Connector>,

	connection: Option<
		ConnectionHandle<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>,
	>,

	options: ConnectableOptions<ConnectorCreator, Connector>,
}

impl<Source, ConnectorCreator, Connector>
	InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorCreator, Connector>) -> Self {
		Self {
			source,
			connector: None,
			connection: None,
			options,
		}
	}

	fn get_connector(&mut self) -> &mut Connector {
		self.connector
			.get_or_insert_with(|| (self.options.connector_creator)())
	}

	fn get_active_connector(&mut self) -> &mut Connector {
		// Remove the connector if it's closed, and only when it's closed
		if self
			.connector
			.as_ref()
			.map(|connector| connector.is_closed())
			.unwrap_or(false)
		{
			self.connector.take();
		}

		self.get_connector()
	}

	fn get_active_connection(
		&mut self,
	) -> Option<ConnectionHandle<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>>
	{
		self.connection
			.as_ref()
			.filter(|connection| !connection.is_closed())
			.cloned()
	}

	fn is_connection_closed(&self) -> bool {
		self.connection
			.as_ref()
			.map(|connection| connection.is_closed())
			.unwrap_or(true)
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.is_connection_closed()
	}

	fn unsubscribe(&mut self) {
		if let Some(connector) = &mut self.connector {
			connector.unsubscribe();
		}
	}
}

impl<Source, ConnectorCreator, Connector> Observable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
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
		let connector = self.get_active_connector();
		connector.subscribe(observer)
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	type ConnectionSubscription =
		Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>;

	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription> {
		self.get_active_connection().unwrap_or_else(|| {
			let mut connector = self.get_connector().clone();

			let mut connection = ConnectionHandle::new(self.source.subscribe(connector.clone()));

			if self.options.unsubscribe_connector_on_disconnect {
				connection.add(Teardown::new(move || connector.unsubscribe()));
			}

			self.connection.replace(connection.clone());

			connection
		})
	}
}
