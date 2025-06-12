use rx_bevy_observable::{Observable, ObservableOutput, Observer};
use rx_bevy_observable_flat::{FlatSubscriber, ForwardFlattener};

// TODO: Try merging pipes together with a single Enum Forwarder over the three forwarders
pub struct FlatPipe<Source, Flattener>
where
	Source: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener,
{
	pub(crate) source_observable: Source,
	pub(crate) flattener: Flattener,
}

impl<Source, Flattener> Clone for FlatPipe<Source, Flattener>
where
	Source: Observable<Out = Flattener::InObservable, OutError = Flattener::InError> + Clone,
	Flattener: ForwardFlattener + Clone,
	Flattener::InObservable: Clone,
{
	fn clone(&self) -> Self {
		Self {
			source_observable: self.source_observable.clone(),
			flattener: self.flattener.clone(),
		}
	}
}

impl<Source, Flattener> FlatPipe<Source, Flattener>
where
	Source: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener,
{
	pub fn new(source_observable: Source, flattener: Flattener) -> Self {
		Self {
			source_observable,
			flattener,
		}
	}
}

impl<Source, Flattener> ObservableOutput for FlatPipe<Source, Flattener>
where
	Source: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener + Clone + 'static,
	Flattener::InObservable: 'static,
{
	type Out = <Flattener::InObservable as ObservableOutput>::Out;
	type OutError = <Flattener::InObservable as ObservableOutput>::OutError;
}

impl<Source, Flattener> Observable for FlatPipe<Source, Flattener>
where
	Source: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener + Clone + 'static,
	Flattener::InObservable: 'static,
{
	type Subscription = Source::Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription {
		let flat_subscriber = FlatSubscriber::new(destination, self.flattener.clone());
		self.source_observable.subscribe(flat_subscriber)
	}
}
