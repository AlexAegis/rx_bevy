use rx_bevy_observable::{InnerSubscription, Subscriber, SubscriptionLike, Teardown};
use slab::Slab;

pub struct MulticastDestination<In, InError> {
	pub(crate) slab: Slab<Box<dyn Subscriber<In = In, InError = InError>>>,
	pub(crate) closed: bool,
	pub(crate) teardown: InnerSubscription,
}

impl<In, InError> MulticastDestination<In, InError> {
	/// Closes this destination and drains its subscribers
	/// It does not do anything with the subscribers as their actions too might
	/// need write access to this destination
	pub fn drain(&mut self) -> Vec<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.closed = true;
		self.slab.drain().collect::<Vec<_>>()
	}

	pub fn take(&mut self, key: usize) -> Option<Box<dyn Subscriber<In = In, InError = InError>>> {
		self.slab.try_remove(key)
	}
}

impl<In, InError> Default for MulticastDestination<In, InError> {
	fn default() -> Self {
		Self {
			slab: Slab::with_capacity(1),
			closed: false,
			teardown: InnerSubscription::new_empty(),
		}
	}
}

impl<In, InError> SubscriptionLike for MulticastDestination<In, InError> {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.teardown.unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.teardown.add(Teardown::Sub(subscription));
	}
}
