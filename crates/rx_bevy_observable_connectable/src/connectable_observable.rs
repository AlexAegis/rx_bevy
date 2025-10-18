use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	Observable, ObservableOutput, SubjectLike, Subscriber, SubscriptionLike, Teardown,
	context::{SubscriptionContext, WithSubscriptionContext},
};
use short_type_name::short_type_name;

use crate::{
	Connectable, ConnectableOptions, ConnectionHandle,
	inner_connectable_observable::InnerConnectableObservable,
};

pub struct ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: 'static,
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
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorCreator, Connector>) -> Self {
		Self {
			connector: Arc::new(RwLock::new(InnerConnectableObservable::new(
				source, options,
			))),
		}
	}
}

impl<Source, ConnectorCreator, Connector> Clone
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	fn clone(&self) -> Self {
		Self {
			connector: self.connector.clone(),
		}
	}
}

impl<Source, ConnectorCreator, Connector> ObservableOutput
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, ConnectorCreator, Connector> WithSubscriptionContext
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Context = Source::Context;
}

impl<Source, ConnectorCreator, Connector> Observable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Subscription = Connector::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		if let Ok(mut lock) = self.connector.write() {
			lock.subscribe(destination, context)
		} else {
			panic!("Poisoned connector lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.connector.read() {
			lock.is_closed()
		} else {
			println!("Poisoned connector lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if let Ok(mut lock) = self.connector.write() {
			lock.unsubscribe(context);
		} else {
			println!("Poisoned connector lock: {}", short_type_name::<Self>());
		}
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Ok(mut lock) = self.connector.write() {
			lock.add_teardown(teardown, context);
		} else {
			println!("Poisoned connector lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: 'static,
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <<Self::ConnectionSubscription as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) -> ConnectionHandle<Self::ConnectionSubscription> {
		if let Ok(mut lock) = self.connector.write() {
			lock.connect(context)
		} else {
			panic!("Poisoned connector lock: {}", short_type_name::<Self>());
		}
	}
}
/*
impl<Source, ConnectorCreator, Connector> Clone
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Clone + Observable,
	ConnectorCreator: Clone + Fn(&mut Connector::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	fn clone(&self) -> Self {
		Self {
			connector: self.connector.clone(),
		}
	}
}
*/
