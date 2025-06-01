use rx_bevy_observable::{DynObserverConnector, Observer};
use slab::Slab;

pub struct MulticastObserver<Instance: DynObserverConnector> {
	pub instance: Instance,
	pub destination: Slab<Box<dyn Observer<In = Instance::Out>>>,
}

impl<Forwarder> MulticastObserver<Forwarder>
where
	Forwarder: DynObserverConnector,
{
	pub fn new(instance: Forwarder) -> Self {
		Self {
			instance,
			destination: Slab::with_capacity(4), // Capacity chosen by fair dice roll.
		}
	}

	pub fn add_destination<Destination: 'static + Observer<In = Forwarder::Out>>(
		&mut self,
		destination: Destination,
	) -> usize {
		self.destination.insert(Box::new(destination))
	}
}

impl<In, Out, F> Observer for MulticastObserver<F>
where
	F: DynObserverConnector<In = In, Out = Out>,
	In: Clone,
{
	type In = In;

	fn on_push(&mut self, value: In) {
		for (_, destination) in self.destination.iter_mut() {
			self.instance
				.push_forward(value.clone(), destination.as_mut());
		}
	}
}
