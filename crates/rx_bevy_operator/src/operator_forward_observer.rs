use rx_bevy_observable::{ConnectorObserver, Observer};

/// An observer that contains a concrete [Destination] and an [OperatorInstance]
/// implementation
/// It's used to connect the internal forwarders of operators to an observer
/// It's mostly only used as an internal detail of operators.
pub struct ForwardObserver<Instance: ConnectorObserver, Destination: Observer<Instance::Out>> {
	pub instance: Instance,
	pub destination: Destination,
}

impl<Instance, Destination> ForwardObserver<Instance, Destination>
where
	Instance: ConnectorObserver,
	Destination: Observer<Instance::Out>,
{
	pub fn new(instance: Instance, destination: Destination) -> Self {
		Self {
			instance,
			destination,
		}
	}
}

impl<In, Out, F, Destination> Observer<In> for ForwardObserver<F, Destination>
where
	F: ConnectorObserver<In = In, Out = Out>,
	Destination: Observer<Out>,
{
	fn on_push(&mut self, value: In) {
		self.instance.push_forward(value, &mut self.destination);
	}
}
