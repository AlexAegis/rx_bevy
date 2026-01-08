use derive_where::derive_where;
use rx_core_common::*;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_observable_connectable::observable::{
	Connectable, ConnectableObservable, ConnectableOptions, ConnectionHandle,
};

#[derive_where(Clone)]
#[derive(RxObservable)]
#[rx_out(<ConnectorProvider::Provided as ObservableOutput>::Out)]
#[rx_out_error(<ConnectorProvider::Provided as ObservableOutput>::OutError)]
pub struct ShareObservable<Source, ConnectorProvider>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
		ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
{
	connectable: ConnectableObservable<
		Source,
		ConnectorProvider,
	>,
	connection: Option<
		ConnectionHandle<
			<ConnectableObservable<
				Source,
			ConnectorProvider,
			> as Connectable>::ConnectionSubscription,
		>,
	>,
}

impl<Source, ConnectorProvider> ShareObservable<Source, ConnectorProvider>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
{
	pub fn new(source: Source, options: ConnectableOptions<ConnectorProvider>) -> Self {
		Self {
			connectable: ConnectableObservable::new(source, options),
			connection: None,
		}
	}

	pub fn is_connected(&self) -> bool {
		self.connection
			.as_ref()
			.map(|connection| !connection.is_closed())
			.unwrap_or(false)
	}

	pub fn connect(
		&mut self,
	) -> ConnectionHandle<
		<ConnectableObservable<Source, ConnectorProvider> as Connectable>::ConnectionSubscription,
	> {
		if let Some(connection) = self.connection.as_ref()
			&& !connection.is_closed()
		{
			return connection.clone();
		}

		let connection = self.connectable.connect();
		self.connection = Some(connection.clone());
		connection
	}

	pub fn disconnect(&mut self) -> bool {
		if let Some(mut connection) = self.connection.take()
			&& !connection.is_closed()
		{
			connection.unsubscribe();
			return true;
		}

		false
	}
}

impl<Source, ConnectorProvider> Observable for ShareObservable<Source, ConnectorProvider>
where
	Source: Observable,
	Source::Out: Clone,
	Source::OutError: Clone,
	ConnectorProvider: 'static + Provider,
	ConnectorProvider::Provided: SubjectLike<In = Source::Out, InError = Source::OutError> + Clone,
{
	type Subscription<Destination>
		= <ConnectableObservable<Source, ConnectorProvider> as Observable>::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		let subscription = self.connectable.subscribe(destination);

		self.connect();

		subscription
	}
}
