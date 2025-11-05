use rx_core_traits::{
	NotSubject, Observable, ObservableOutput, SubjectLike, Subscriber, SubscriptionCollection,
	SubscriptionContext, SubscriptionLike, Teardown, WithSubscriptionContext,
};

use crate::observable::{Connectable, ConnectableOptions, ConnectionHandle};

pub struct InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: 'static,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,

	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Option<Connector>,

	connection: Option<ConnectionHandle<Source::Subscription>>,

	options: ConnectableOptions<ConnectorCreator, Connector>,
}

impl<Source, ConnectorCreator, Connector>
	InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorCreator, Connector>) -> Self {
		Self {
			source,
			connector: None,
			connection: None,
			options,
		}
	}

	fn get_connector(
		&mut self,
		context: &mut <Connector::Context as SubscriptionContext>::Item<'_, '_>,
	) -> &mut Connector {
		self.connector
			.get_or_insert_with(|| (self.options.connector_creator)(context))
	}

	fn get_active_connector(
		&mut self,
		context: &mut <Connector::Context as SubscriptionContext>::Item<'_, '_>,
	) -> &mut Connector {
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

	fn get_active_connection(&mut self) -> Option<ConnectionHandle<Source::Subscription>> {
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
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, ConnectorCreator, Connector> Observable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type IsSubject = NotSubject;
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
		let connector = self.get_active_connector(context);
		connector.subscribe(destination, context)
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ Clone
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
	Source::Subscription: 'static,
{
	type ConnectionSubscription = Source::Subscription;

	fn connect(
		&mut self,
		context: &mut <<Self::ConnectionSubscription as WithSubscriptionContext>::Context as SubscriptionContext>::Item<'_, '_>,
	) -> ConnectionHandle<Self::ConnectionSubscription> {
		self.get_active_connection().unwrap_or_else(|| {
			let connector = self.get_connector(context).clone();

			let mut connection =
				ConnectionHandle::new(self.source.subscribe(connector.clone(), context), context);

			if self.options.unsubscribe_connector_on_disconnect {
				connection.add(connector, context);
			}

			self.connection.replace(connection.clone());

			connection
		})
	}
}

impl<Source, ConnectorCreator, Connector> WithSubscriptionContext
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	type Context = Connector::Context;
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn(&mut <Source::Context as SubscriptionContext>::Item<'_, '_>) -> Connector,
	Connector: 'static
		+ SubjectLike<In = Source::Out, InError = Source::OutError, Context = Source::Context>,
	<Connector as Observable>::Subscription: SubscriptionLike<Context = Source::Context>,
{
	fn is_closed(&self) -> bool {
		self.is_connection_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if let Some(connector) = &mut self.connector {
			connector.unsubscribe(context);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(connector) = &mut self.connector {
			connector.add_teardown(teardown, context);
		}
	}
}
