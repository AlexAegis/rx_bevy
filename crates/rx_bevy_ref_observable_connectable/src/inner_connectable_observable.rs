use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, SubjectLike, Subscriber, SubscriptionCollection,
	SubscriptionLike,
};

use crate::{Connectable, ConnectableOptions};

pub struct InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Option<Connector>,

	connection: Option<Source::Subscription>,

	options: ConnectableOptions<ConnectorCreator, Connector>,
}

impl<Source, ConnectorCreator, Connector>
	InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription: Clone,
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
			.get_or_insert_with(&self.options.connector_creator)
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

	fn get_active_connection(&mut self) -> Option<Source::Subscription> {
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

impl<Source, ConnectorCreator, Connector> ObservableOutput
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, ConnectorCreator, Connector> Observable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription: Clone,
{
	type Subscription = Connector::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	{
		let connector = self.get_active_connector();
		connector.subscribe(destination, context)
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
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
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as SignalContext>::Context,
	) -> Self::ConnectionSubscription {
		self.get_active_connection().unwrap_or_else(|| {
			let connector = self.get_connector().clone();

			let mut connection = self.source.subscribe(connector.clone(), context);

			if self.options.unsubscribe_connector_on_disconnect {
				connection.add(connector, context);
			}

			self.connection.replace(connection.clone());

			connection
		})
	}
}

impl<Source, ConnectorCreator, Connector> SignalContext
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	type Context = Connector::Context;
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription: Clone,
{
	fn is_closed(&self) -> bool {
		self.is_connection_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Some(connector) = &mut self.connector {
			connector.unsubscribe(context);
		}
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionCollection
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
	Source::Subscription: Clone,
	Connector: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<rx_bevy_core::Teardown<S, S::Context>>,
	{
		if let Some(connector) = &mut self.connector {
			connector.add(subscription, context);
		}
	}
}
