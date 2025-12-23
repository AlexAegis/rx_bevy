use std::sync::{Arc, Mutex};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, SubjectLike, Subscriber, SubscriptionLike,
	TeardownCollection, TeardownCollectionExtension, UpgradeableObserver,
};

use crate::{
	internal::{
		Connection, ConnectionOptions, ConnectionState, ConnectionSubscriber, ConnectorState,
	},
	observable::{Connectable, ConnectableOptions, ConnectionHandle},
};

pub type ConnectionSubscription<Source, Connector> =
	<Source as Observable>::Subscription<ConnectionSubscriber<Connector>>;

#[derive(RxObservable)]
#[rx_out(Connector::Out)]
#[rx_out_error(Connector::OutError)]
pub struct ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	ConnectionSubscription<Source, Connector>: 'static + TeardownCollection,
{
	/// The only reason this field is behind an `Arc<RwLock>` is to be able to
	/// pipe operators over a connectable observable.
	/// ? It could very well be the case that piped operators are not even needed
	/// ? for this ConnectableObservable as it is a low level component of other operators. (share)
	/// ? if that's the case, revisit this and remove the arc
	connector: Arc<Mutex<ConnectorState<Source, Connector>>>,

	connection: Arc<Mutex<Connection<ConnectionSubscription<Source, Connector>>>>,
	connection_state: Arc<Mutex<ConnectionState>>,
}

impl<Source, Connector> ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	ConnectionSubscription<Source, Connector>: 'static + TeardownCollection,
{
	pub fn new(source: Source, options: ConnectableOptions<Connector>) -> Self {
		let connection_state = Arc::new(Mutex::new(ConnectionState::new(
			ConnectionOptions::from_connectable_options(&options),
		)));
		let connection = Arc::new(Mutex::new(Connection::default()));
		Self {
			connector: Arc::new(Mutex::new(ConnectorState::new(
				source,
				connection.clone(),
				connection_state.clone(),
				options,
			))),
			connection,
			connection_state,
		}
	}
}

impl<Source, Connector> Clone for ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	fn clone(&self) -> Self {
		Self {
			connector: self.connector.clone(),
			connection: self.connection.clone(),
			connection_state: self.connection_state.clone(),
		}
	}
}

impl<Source, Connector> Observable for ConnectableObservable<Source, Connector>
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
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut subscription = {
			let mut connector = self.connector.lock_ignore_poison();
			connector.get_connector().subscribe(observer.upgrade())
		};

		if !subscription.is_closed() {
			self.connection_state
				.lock_ignore_poison()
				.increment_subscriber_count();

			let connection_state_clone = self.connection_state.clone();
			let connection_clone = self.connection.clone();
			subscription.add_fn(move || {
				let connection_to_disconnect = {
					let mut connection_state = connection_state_clone.lock_ignore_poison();

					// If, and only if when `disconnect_when_ref_count_zero` is set,
					// and the ref count dropped to zero this function returns true
					let should_disconnect = connection_state.decrement_subscriber_count();

					if should_disconnect {
						connection_clone.lock_ignore_poison().take_connection()
					} else {
						None
					}
				};

				if let Some(mut connection) = connection_to_disconnect
					&& !connection.is_closed()
				{
					connection.unsubscribe();
				};
			});
		}

		subscription
	}
}

impl<Source, Connector> Connectable for ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	type ConnectionSubscription = ConnectionSubscription<Source, Connector>;

	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription> {
		self.connector.lock_ignore_poison().connect()
	}

	#[inline]
	fn disconnect(&mut self) -> bool {
		self.connector.lock_ignore_poison().disconnect()
	}

	#[inline]
	fn is_connected(&self) -> bool {
		self.connection.lock_ignore_poison().is_connected()
	}

	#[inline]
	fn reset(&mut self) {
		self.connector.lock_ignore_poison().reset();
	}
}
