use rx_bevy_observable::{DynConnectorObserver, Observer};

pub struct MulticastForwardObserver<Instance: DynConnectorObserver> {
	pub instance: Instance,
	pub destinations: Vec<Box<dyn Observer<Instance::Out>>>,
}

impl<Instance> MulticastForwardObserver<Instance>
where
	Instance: DynConnectorObserver,
{
	pub fn new(instance: Instance) -> Self {
		Self {
			instance,
			destinations: Vec::new(),
		}
	}

	pub fn add_destination<Destination: 'static + Observer<Instance::Out>>(
		&mut self,
		destination: Destination,
	) {
		self.destinations.push(Box::new(destination));
	}
}

impl<In, Out, F> Observer<In> for MulticastForwardObserver<F>
where
	F: DynConnectorObserver<In = In, Out = Out>,
	In: Clone,
{
	fn on_push(&mut self, value: In) {
		for destination in self.destinations.iter_mut() {
			self.instance
				.push_forward(value.clone(), destination.as_mut());
		}
	}
}
