use std::sync::{Arc, Mutex};

use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	LockWithPoisonBehavior, Observable, SubjectLike, Subscriber, SubscriptionLike,
	TeardownCollection, TeardownCollectionExtension, UpgradeableObserver,
};

use crate::{
	internal::{ConnectionOptions, ConnectionState, ConnectorState},
	observable::{Connectable, ConnectableOptions, ConnectionHandle},
};

#[derive(RxObservable)]
#[rx_out(Connector::Out)]
#[rx_out_error(Connector::OutError)]
pub struct ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	/// The only reason this field is behind an `Arc<RwLock>` is to be able to
	/// pipe operators over a connectable observable.
	/// ? It could very well be the case that piped operators are not even needed
	/// ? for this ConnectableObservable as it is a low level component of other operators. (share)
	/// ? if that's the case, revisit this and remove the arc
	connector: Arc<Mutex<ConnectorState<Source, Connector>>>,

	connection: Arc<
		Mutex<ConnectionState<Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>>>,
	>,
}

impl<Source, Connector> ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + Clone + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>:
		'static + TeardownCollection,
{
	pub fn new(source: Source, options: ConnectableOptions<Connector>) -> Self {
		let connection = Arc::new(Mutex::new(ConnectionState::new(
			ConnectionOptions::from_connectable_options(&options),
		)));
		Self {
			connector: Arc::new(Mutex::new(ConnectorState::new(
				source,
				connection.clone(),
				options,
			))),
			connection,
		}
	}

	pub fn is_closed(&self) -> bool {
		self.connector.is_closed()
	}

	pub fn unsubscribe(&mut self) {
		self.connector.unsubscribe();
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

			connector.subscribe(observer.upgrade())
		};

		if !subscription.is_closed() {
			self.connection
				.lock_ignore_poison()
				.increment_subscriber_count();

			let connection_state_clone = self.connection.clone();
			subscription.add_fn(move || {
				let connection_to_disconnect = {
					let mut connection_state = connection_state_clone.lock_ignore_poison();
					if connection_state.decrement_subscriber_count() {
						connection_state.get_connection()
					} else {
						None
					}
				};

				if let Some(mut connection) = connection_to_disconnect {
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
	type ConnectionSubscription =
		<Source as Observable>::Subscription<<Connector as UpgradeableObserver>::Upgraded>;

	fn connect(&mut self) -> ConnectionHandle<Self::ConnectionSubscription> {
		match self.connector.lock() {
			Ok(mut connector) => connector.connect(),
			Err(poison_error) => {
				let error_message =
					format!("Poisoned lock encountered, unable to subscribe! {poison_error:?}");
				poison_error.into_inner().unsubscribe();
				panic!("{}", error_message)
			}
		}
	}

	fn disconnect(&mut self) -> bool {
		let mut s = self.connector.lock_ignore_poison();
		s.disconnect()
	}

	fn is_connected(&self) -> bool {
		self.connector.lock_ignore_poison().is_connected()
	}

	fn reset(&mut self) {
		self.connector.lock_ignore_poison().reset();
	}
}
