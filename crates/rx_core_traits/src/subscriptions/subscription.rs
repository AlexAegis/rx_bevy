use crate::{Subscriber, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection};

pub struct Subscription<Destination>
where
	Destination: Subscriber,
{
	teardown: SubscriptionData,
	destination: Destination,
}

impl<Destination> Subscription<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Destination> SubscriptionLike for Subscription<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.teardown.unsubscribe();
			self.destination.unsubscribe();
		}
	}
}

impl<Destination> TeardownCollection for Subscription<Destination>
where
	Destination: Subscriber,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if self.is_closed() {
			// If this subscription is already closed, the newly added teardown
			// is immediately executed.
			teardown.execute();
		} else {
			self.teardown.add_teardown(teardown);
		}
	}
}

impl<Destination> Drop for Subscription<Destination>
where
	Destination: Subscriber,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.unsubscribe();
		}
	}
}
