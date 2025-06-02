use rx_bevy_observable::{Observer, ObserverConnector};

/// An observer that contains a concrete [Destination] and an [OperatorInstance]
/// implementation
/// It's used to connect the internal forwarders of operators to an observer
/// It's mostly only used as an internal detail of operators.
/// TODO: Rename to Subscriber
pub struct ForwardObserver<Instance: ObserverConnector, Destination: Observer<In = Instance::Out>> {
	pub instance: Instance,
	pub destination: Destination,
	pub closed: bool,
}

impl<Instance, Destination> ForwardObserver<Instance, Destination>
where
	Instance: ObserverConnector,
	Destination: Observer<In = Instance::Out, Error = Instance::OutError>,
{
	pub fn new(instance: Instance, destination: Destination) -> Self {
		Self {
			instance,
			destination,
			closed: false,
		}
	}
}

impl<In, Out, InError, OutError, F, Destination> Observer for ForwardObserver<F, Destination>
where
	F: ObserverConnector<In = In, Out = Out, InError = InError, OutError = OutError>,
	Destination: Observer<In = Out, Error = OutError>,
{
	type In = In;
	type Error = InError;

	fn on_push(&mut self, value: In) {
		if !self.closed {
			self.instance.push_forward(value, &mut self.destination);
		}
	}

	fn on_error(&mut self, error: Self::Error) {
		if !self.closed {
			self.closed = true;
			self.instance.error_forward(error, &mut self.destination);
		}
	}

	fn on_complete(&mut self) {
		if !self.closed {
			self.closed = true;
			self.instance.complete_forward(&mut self.destination);
		}
	}
}
