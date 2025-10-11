use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, SubjectLike, Subscriber, SubscriptionCollection,
	SubscriptionHandle, SubscriptionLike, Teardown, WithContext,
};

use crate::{Connectable, ConnectableOptions};

pub struct InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: Clone,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Option<Connector>,

	connection: Option<SubscriptionHandle<Source::Subscription>>,

	options: ConnectableOptions<ConnectorCreator, Connector>,
}

impl<Source, ConnectorCreator, Connector>
	InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
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

	fn get_connector(&mut self, context: &mut Connector::Context) -> &mut Connector {
		self.connector
			.get_or_insert_with(|| (self.options.connector_creator)(context))
	}

	fn get_active_connector(&mut self, context: &mut Connector::Context) -> &mut Connector {
		// Remove the connector if it's closed, and only when it's closed
		if self
			.connector
			.as_ref()
			.map(|connector| connector.is_closed())
			.unwrap_or(false)
		{
			self.connector.take();
		}

		self.get_connector(context)
	}

	fn get_active_connection(&mut self) -> Option<SubscriptionHandle<Source::Subscription>> {
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
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: Clone,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, ConnectorCreator, Connector> Observable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: Clone,
{
	type Subscription = Connector::Subscription;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let connector = self.get_active_connector(context);
		connector.subscribe(destination, context)
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: Clone,
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <Self::ConnectionSubscription as WithContext>::Context,
	) -> SubscriptionHandle<Self::ConnectionSubscription> {
		self.get_active_connection().unwrap_or_else(|| {
			let connector = self.get_connector(context).clone();

			let mut connection = self.source.subscribe(connector.clone(), context);

			if self.options.unsubscribe_connector_on_disconnect {
				connection.add(connector, context);
			}

			self.connection.replace(connection.clone());

			connection
		})
	}
}

impl<Source, ConnectorCreator, Connector> WithContext
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Source::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: Clone,
{
	type Context = Connector::Context;
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut Connector::Context) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
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

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if let Some(connector) = &mut self.connector {
			connector.add_teardown(teardown, context);
		}
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Some(connector) = &mut self.connector {
			connector.get_context_to_unsubscribe_on_drop()
		} else {
			println!("oh no");
			Self::Context::create_context_to_unsubscribe_on_drop()
		}
	}
}
