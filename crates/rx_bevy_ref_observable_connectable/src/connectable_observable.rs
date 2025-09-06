use std::sync::{Arc, Mutex};

use rx_bevy_core::{
	DropContext, Observable, ObservableOutput, SignalContext, SubjectLike, SubscriptionCollection,
	SubscriptionLike, Teardown, UpgradeableObserver,
};

use crate::{
	Connectable, ConnectableOptions, inner_connectable_observable::InnerConnectableObservable,
};

pub struct ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	connector: Arc<Mutex<InnerConnectableObservable<Source, ConnectorCreator, Connector>>>,
}

impl<Source, ConnectorCreator, Connector> ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription: Clone,
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
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, ConnectorCreator, Connector> Observable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static
		+ SubjectLike<
			In = Source::Out,
			InError = Source::OutError,
			Context = <Source::Subscription as SignalContext>::Context,
		>,
	Source::Subscription: Clone,
{
	type Subscription = Connector::Subscription;

	fn subscribe<
		Destination: 'static
			+ UpgradeableObserver<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription {
		let mut connector = self.connector.lock().expect("cant lock");
		connector.subscribe(destination, context)
	}
}

impl<Source, ConnectorCreator, Connector> SignalContext
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static
		+ SubjectLike<
			In = Source::Out,
			InError = Source::OutError,
			Context = <Source::Subscription as SignalContext>::Context,
		>,
	Source::Subscription: Clone,
{
	type Context = <Source::Subscription as SignalContext>::Context;
}

impl<Source, ConnectorCreator, Connector> SubscriptionCollection
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static
		+ SubjectLike<
			In = Source::Out,
			InError = Source::OutError,
			Context = <Source::Subscription as SignalContext>::Context,
		>,
	Source::Subscription: Clone,
	Connector: SubscriptionCollection,
{
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		let mut connector = self.connector.lock().expect("lockable");
		connector.add(subscription, context);
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static
		+ SubjectLike<
			In = Source::Out,
			InError = Source::OutError,
			Context = <Source::Subscription as SignalContext>::Context,
		>,
	Source::Subscription: Clone,
{
	fn is_closed(&self) -> bool {
		let connector = self.connector.lock().expect("lockable");
		connector.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		let mut connector = self.connector.lock().expect("lockable");
		connector.unsubscribe(context);
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static
		+ SubjectLike<
			In = Source::Out,
			InError = Source::OutError,
			Context = <Source::Subscription as SignalContext>::Context,
		>,
	Source::Subscription: Clone + SubscriptionCollection,
	<Source::Subscription as SignalContext>::Context: DropContext,
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as SignalContext>::Context,
	) -> Self::ConnectionSubscription {
		let mut connector = self.connector.lock().expect("cant lock");
		connector.connect(context)
	}
}

impl<Source, ConnectorCreator, Connector> Clone
	for ConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Clone + Observable,
	ConnectorCreator: Clone + Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	fn clone(&self) -> Self {
		Self {
			connector: self.connector.clone(),
		}
	}
}
