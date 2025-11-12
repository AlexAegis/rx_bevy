use std::sync::{Arc, RwLock};

use disqualified::ShortName;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Observable, SubjectLike, Subscriber, SubscriptionContext, SubscriptionLike,
	UpgradeableObserver, WithSubscriptionContext,
};

use crate::{
	InnerConnectableObservable,
	observable::{Connectable, ConnectableOptions, ConnectionHandle},
};

#[derive(RxObservable)]
#[rx_out(Connector::Out)]
#[rx_out_error(Connector::OutError)]
#[rx_context(Source::Context)]
pub struct ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>: 'static,
{
	/// The only reason this field is behind an `Arc<RwLock>` is to be able to
	/// pipe operators over a connectable observable.
	/// ? It could very well be the case that piped operators are not even needed
	/// ? for this ConnectableObservable as it is a low level component of other operators. (share)
	/// ? if that's the case, revisit this and remove the arc
	connector: Arc<RwLock<InnerConnectableObservable<Source, ConnectorCreator, Connector>>>,
}

impl<Source, ConnectorCreator, Connector> ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>: 'static,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorCreator, Connector>) -> Self {
		Self {
			connector: Arc::new(RwLock::new(InnerConnectableObservable::new(
				source, options,
			))),
		}
	}

	pub fn is_closed(&self) -> bool {
		if let Ok(lock) = self.connector.read() {
			lock.is_closed()
		} else {
			println!("Poisoned connector lock: {}", ShortName::of::<Self>());
			true
		}
	}

	pub fn unsubscribe(
		&mut self,
		context: &mut <Connector::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Ok(mut lock) = self.connector.write() {
			lock.unsubscribe(context);
		} else {
			println!("Poisoned connector lock: {}", ShortName::of::<Self>());
		}
	}
}

impl<Source, ConnectorCreator, Connector> Clone
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>: 'static,
{
	fn clone(&self) -> Self {
		Self {
			connector: self.connector.clone(),
		}
	}
}

impl<Source, ConnectorCreator, Connector> Observable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>: 'static,
{
	type Subscription<Destination>
		= Connector::Subscription<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		let destination = observer.upgrade();
		if let Ok(mut lock) = self.connector.write() {
			lock.subscribe(destination, context)
		} else {
			panic!("Poisoned connector lock: {}", ShortName::of::<Self>());
		}
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>: 'static,
{
	type ConnectionSubscription =
		Source::Subscription<<Connector as UpgradeableObserver>::Upgraded>;

	fn connect(
		&mut self,
		context: &mut <<Self::ConnectionSubscription as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) -> ConnectionHandle<Self::ConnectionSubscription> {
		if let Ok(mut lock) = self.connector.write() {
			lock.connect(context)
		} else {
			panic!("Poisoned connector lock: {}", ShortName::of::<Self>());
		}
	}
}
