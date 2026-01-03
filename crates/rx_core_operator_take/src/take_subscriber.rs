use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionLike};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	count: usize,
}

impl<Destination> TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(mut destination: Destination, count: usize) -> Self {
		if count == 0 {
			destination.complete();
		}
		Self { destination, count }
	}
}

impl<Destination> Observer for TakeSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() && self.count > 0 {
			self.count -= 1;
			self.destination.next(next);

			if self.count == 0 {
				self.complete();
			}
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		if !self.is_closed() {
			self.destination.error(error);
			self.unsubscribe();
		}
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			self.destination.complete();
			self.unsubscribe();
		}
	}
}
