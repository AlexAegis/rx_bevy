use rx_bevy_observable::{Observable, ObservableOutput, Observer, SubjectLike, Subscription};

pub trait Connectable: Observable {
	fn connect(&mut self) -> Subscription;
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

impl<Source, Connector> ConnectableObservable<Source, Connector>
where
	Source: Observable,
	Connector: 'static + SubjectLike<In = Source::Out, InError = Source::OutError>,
{
	pub fn new(source: Source, connector: Connector) -> Self {
		Self { source, connector }
	}
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
