use rx_bevy_observable::{Observable, ObservableOutput, Observer, Subscription, SubscriptionLike};

pub trait Connectable: Observable {
	fn connect(&mut self) -> Subscription;
}

pub trait SubjectLike: Clone + Observable + Observer + SubscriptionLike {}

pub struct ConnectableObservableConfiguration<Connector, ConnectorCreator>
where
	ConnectorCreator: Fn() -> Connector,
	Connector: 'static + SubjectLike,
{
	connector_creator: ConnectorCreator,
}

/// TODO: Should be part of core or its own
pub struct ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	/// Upon connection, the connector subject will subscribe to this source
	/// observable
	source: Source,
	/// Upon subscription this connector subject is what will be used as the
	/// source
	connector: Connector,
}

impl<Source, Connector> ObservableOutput for ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	type Out = Connector::Out;
	type OutError = Connector::OutError;
}

impl<Source, Connector> Observable for ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		self.connector.subscribe(destination)
	}
}

impl<Source, Connector> Connectable for ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	fn connect(&mut self) -> Subscription {
		self.source.subscribe(self.connector.clone())
	}
}
