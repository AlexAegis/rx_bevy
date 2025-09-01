use rx_bevy_core::{
	Observable, ObservableOutput, SubjectLike, Subscription, SubscriptionLike, Teardown,
	UpgradeableObserver,
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

	connection: Option<Subscription>,

	options: ConnectableOptions<ConnectorCreator, Connector>,
}

impl<Source, ConnectorCreator, Connector>
	InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
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

	fn get_active_connection(&mut self) -> Option<Subscription> {
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
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let connector = self.get_active_connector();
		connector.subscribe(destination)
	}
}

impl<Source, ConnectorCreator, Connector> Connectable
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	fn connect(&mut self) -> Subscription {
		self.get_active_connection().unwrap_or_else(|| {
			let mut connector = self.get_connector().clone();

			let mut connection = self.source.subscribe(connector.clone());

			if self.options.unsubscribe_connector_on_disconnect {
				connection.add(Teardown::new(Box::new(move || {
					connector.unsubscribe();
				})));
			}

			self.connection.replace(connection.clone());

			connection
		})
	}
}

impl<Source, ConnectorCreator, Connector> SubscriptionLike
	for InnerConnectableObservable<Source, ConnectorCreator, Connector>
where
	Source: Observable,
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	fn is_closed(&self) -> bool {
		self.is_connection_closed()
	}

	fn unsubscribe(&mut self) {
		if let Some(connector) = &mut self.connector {
			connector.unsubscribe();
		}
	}

	#[inline]
	fn add(&mut self, subscription: Box<dyn SubscriptionLike>) {
		if let Some(connector) = &mut self.connector {
			connector.add(subscription);
		}
	}
}
