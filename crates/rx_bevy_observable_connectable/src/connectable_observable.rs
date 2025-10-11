use std::sync::{Arc, Mutex};

use rx_bevy_core::{
	Observable, ObservableOutput, SubjectLike, Subscriber, SubscriptionLike, Teardown,
	WithSubscriptionContext,
};

use crate::{
	Connectable, ConnectableOptions, ConnectionHandle,
	inner_connectable_observable::InnerConnectableObservable,
};

pub struct ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	connector: Arc<Mutex<InnerConnectableObservable<Source, ConnectorCreator, Connector>>>,
}

impl<Source, ConnectorCreator, Connector> ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorCreator, Connector>) -> Self {
		Self {
			connector: Arc::new(Mutex::new(InnerConnectableObservable::new(source, options))),
		}
	}
}

impl<Source, ConnectorCreator, Connector> ObservableOutput
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
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
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Context = Source::Context;
}

impl<Source, ConnectorCreator, Connector> Observable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Subscription = Connector::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		print!("connectable subscribe about to lock..");
		let mut connector = self.connector.lock().expect("cant lock");
		println!(".. locked!");

		connector.subscribe(destination, context)
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	fn is_closed(&self) -> bool {
		print!("connectable is_closed about to lock..");
		let connector = self.connector.lock().expect("lockable");
		println!(".. locked!");

		connector.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		print!("connectable unsubscribe about to lock..");
		let mut connector = self.connector.lock().expect("lockable");
		println!(".. locked!");

		connector.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		print!("connectable add_teardown about to lock..");

		let mut connector = self.connector.lock().expect("lockable");
		println!(".. locked!");

		connector.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		print!("connectable get_unsubscribe_context about to lock..");

		let mut connector = self.connector.lock().expect("lockable");
		println!(".. locked!");

		connector.get_context_to_unsubscribe_on_drop()
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Connector::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: 'static,
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithSubscriptionContext>::Context,
	) -> ConnectionHandle<Self::ConnectionSubscription> {
		print!("connectable connect about to lock..");

		let mut connector = self.connector.lock().expect("cant lock");
		println!(".. locked!");

		connector.connect(context)
	}
}

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
