use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionLike, Tick,
};
use slab::Slab;

pub struct MulticastDestination<In, InError, Context> {
	pub(crate) slab: Slab<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>>,
	pub(crate) closed: bool,
}

impl<In, InError, Context> MulticastDestination<In, InError, Context> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub fn drain(
		&mut self,
	) -> Vec<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>> {
		self.closed = true;
		self.slab.drain().collect::<Vec<_>>()
	}

	pub fn take(
		&mut self,
		key: usize,
	) -> Option<Box<dyn Subscriber<In = In, InError = InError, Context = Context>>> {
		self.slab.try_remove(key)
	}

	pub fn multicast_subscribe<
		Destination: 'static + Subscriber<In = In, InError = InError, Context = Context>,
	>(
		&mut self,
		subscriber: Destination,
	) -> usize {
		let entry = self.slab.vacant_entry();
		let key = entry.key();
		entry.insert(Box::new(subscriber));
		key
	}
}

impl<In, InError, Context> Observer for MulticastDestination<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		for (_, destination) in self.slab.iter_mut() {
			destination.next(next.clone(), context);
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		let destinations = self.drain();
		for mut destination in destinations {
			destination.error(error.clone(), context);
			destination.unsubscribe(context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		let mut destinations = self.drain();
		for destination in destinations.iter_mut() {
			destination.complete(context);
			destination.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		for (_, destination) in self.slab.iter_mut() {
			destination.tick(tick.clone(), context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for MulticastDestination<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		for mut destination in self.drain() {
			destination.unsubscribe(context);
		}
	}
}

impl<In, InError, Context> ObserverInput for MulticastDestination<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> ObservableOutput for MulticastDestination<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Context> Default for MulticastDestination<In, InError, Context> {
	fn default() -> Self {
		Self {
			slab: Slab::with_capacity(1),
			closed: false,
		}
	}
}

impl<In, InError, Context> SignalContext for MulticastDestination<In, InError, Context> {
	type Context = Context;
}
