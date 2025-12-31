use std::sync::{Arc, Mutex};

use derive_where::derive_where;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, ObservableOutput, Provider, SubjectLike, Subscriber,
	SubscriptionLike, TeardownCollection, TeardownCollectionExtension, UpgradeableObserver,
};

use crate::{
	internal::{
		Connection, ConnectionOptions, ConnectionState, ConnectionSubscriber, ConnectorState,
	},
	observable::{Connectable, ConnectableOptions, ConnectionHandle},
};

pub type ConnectionSubscription<Source, Connector> =
	<Source as Observable>::Subscription<ConnectionSubscriber<Connector>>;

#[derive_where(Clone)]
#[derive(RxObservable)]
#[rx_out(<ConnectorProvider::Provided as ObservableOutput>::Out)]
#[rx_out_error(<ConnectorProvider::Provided as ObservableOutput>::OutError)]
pub struct ConnectableObservable<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	/// The only reason this field is behind an `Arc<RwLock>` is to be able to
	/// pipe operators over a connectable observable.
	/// ? It could very well be the case that piped operators are not even needed
	/// ? for this ConnectableObservable as it is a low level component of other operators. (share)
	/// ? if that's the case, revisit this and remove the arc
	connector: Arc<Mutex<ConnectorState<Source, ConnectorProvider>>>,

	connection: Arc<Mutex<Connection<ConnectionSubscription<Source, ConnectorProvider::Provided>>>>,
	connection_state: Arc<Mutex<ConnectionState>>,
}

impl<Source, ConnectorProvider> ConnectableObservable<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorProvider>) -> Self {
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

impl<Source, ConnectorProvider> Observable for ConnectableObservable<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	type Subscription<Destination>
		= <ConnectorProvider::Provided as Observable>::Subscription<Destination>
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

impl<Source, ConnectorProvider> Connectable for ConnectableObservable<Source, ConnectorProvider>
where
	Source: Observable,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
	ConnectionSubscription<Source, ConnectorProvider::Provided>: 'static + TeardownCollection,
{
	type ConnectionSubscription = ConnectionSubscription<Source, ConnectorProvider::Provided>;

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

#[cfg(test)]
mod test {
	use std::sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	};

	use rx_core_subject_publish::subject::PublishSubject;
	use rx_core_traits::{LockWithPoisonBehavior, ProvideWithDefault, TeardownCollectionExtension};

	use crate::observable::{Connectable, ConnectableObservable, ConnectableOptions};

	#[test]
	fn the_connection_subscription_should_be_unsubscribed_on_disconnect() {
		let source = PublishSubject::<usize, &'static str>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider:
					ProvideWithDefault::<PublishSubject<usize, &'static str>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		connectable_observable.connect();

		let mut connection = connectable_observable
			.connection
			.lock_ignore_poison()
			.get_active_connection()
			.unwrap();

		let teardown_called = Arc::new(AtomicBool::new(false));
		let teardown_called_clone = teardown_called.clone();
		connection.add_fn(move || teardown_called_clone.store(true, Ordering::Relaxed));

		connectable_observable.disconnect();

		assert!(teardown_called.load(Ordering::Relaxed))
	}
}
