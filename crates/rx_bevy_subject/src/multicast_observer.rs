use rx_bevy_observable::{DynForwarder, Observer, ObserverInput};
use slab::Slab;

pub struct MulticastObserver<Instance: DynForwarder> {
	pub instance: Instance,
	pub destination: Slab<Box<dyn Observer<In = Instance::Out, InError = Instance::OutError>>>,
	pub closed: bool,
}

impl<Forwarder> MulticastObserver<Forwarder>
where
	Forwarder: DynForwarder,
{
	pub fn new(instance: Forwarder) -> Self {
		Self {
			instance,
			destination: Slab::with_capacity(4), // Capacity chosen by fair dice roll.
			closed: false,
		}
	}

	pub fn add_destination<
		Destination: 'static + Observer<In = Forwarder::Out, InError = Forwarder::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> usize {
		self.destination.insert(Box::new(destination))
	}
}

impl<In, Out, InError, F> ObserverInput for MulticastObserver<F>
where
	F: DynForwarder<In = In, Out = Out, InError = InError>,
	In: Clone,
	InError: Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, Out, InError, F> Observer for MulticastObserver<F>
where
	F: DynForwarder<In = In, Out = Out, InError = InError>,
	In: Clone,
	InError: Clone,
{
	fn next(&mut self, next: In) {
		if !self.closed {
			for (_, destination) in self.destination.iter_mut() {
				self.instance
					.next_forward(next.clone(), destination.as_mut());
			}
		}
	}

	fn error(&mut self, error: Self::InError) {
		if !self.closed {
			self.closed = true;
			for (_, destination) in self.destination.iter_mut() {
				self.instance
					.error_forward(error.clone(), destination.as_mut());
			}
		}
	}

	fn complete(&mut self) {
		if !self.closed {
			self.closed = true;
			for (_, destination) in self.destination.iter_mut() {
				self.instance.complete_forward(destination.as_mut());
			}
		}
	}
}
