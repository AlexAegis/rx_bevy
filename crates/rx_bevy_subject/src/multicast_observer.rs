use rx_bevy_observable::{
	Observer, ObserverInput, Subscriber, Subscription, forwarders::DynForwarder,
};
use slab::Slab;

pub struct MulticastSubscriber<Instance: DynForwarder> {
	pub instance: Instance,
	pub destination: Slab<Box<dyn Subscriber<In = Instance::Out, InError = Instance::OutError>>>,
	pub closed: bool,
}

impl<Forwarder> MulticastSubscriber<Forwarder>
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
		Destination: 'static + Subscriber<In = Forwarder::Out, InError = Forwarder::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> usize {
		self.destination.insert(Box::new(destination))
	}
}

impl<In, Out, InError, F> ObserverInput for MulticastSubscriber<F>
where
	F: DynForwarder<In = In, Out = Out, InError = InError>,
	In: 'static + Clone,
	InError: 'static + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, Out, InError, F> Observer for MulticastSubscriber<F>
where
	F: DynForwarder<In = In, Out = Out, InError = InError>,
	In: 'static + Clone,
	InError: 'static + Clone,
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

impl<In, Out, InError, F> Subscription for MulticastSubscriber<F>
where
	F: DynForwarder<In = In, Out = Out, InError = InError>,
	In: Clone,
	InError: Clone,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
		//for destination in self.destination.drain() {}
	}
}
