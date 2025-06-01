use rx_bevy_observable::{Observer, ObserverConnector};

/// An observer that contains a concrete [Destination] and an [OperatorInstance]
/// implementation
/// It's used to connect the internal forwarders of operators to an observer
/// It's mostly only used as an internal detail of operators.
pub struct ForwardObserver<Instance: ObserverConnector, Destination: Observer<In = Instance::Out>> {
	pub instance: Instance,
	pub destination: Destination,
}

impl<Instance, Destination> ForwardObserver<Instance, Destination>
where
	Instance: ObserverConnector,
	Destination: Observer<In = Instance::Out>,
{
	pub fn new(instance: Instance, destination: Destination) -> Self {
		Self {
			instance,
			destination,
		}
	}
}

impl<In, Out, F, Destination> Observer for ForwardObserver<F, Destination>
where
	F: ObserverConnector<In = In, Out = Out>,
	Destination: Observer<In = Out>,
{
	type In = In;

	fn on_push(&mut self, value: In) {
		self.instance.push_forward(value, &mut self.destination);
	}
}
