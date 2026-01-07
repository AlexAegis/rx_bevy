use rx_core_common::{Observer, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct SkipSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	count: usize,
}

impl<Destination> SkipSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination, count: usize) -> Self {
		Self { destination, count }
	}
}

impl<Destination> Observer for SkipSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if self.count == 0 {
			self.destination.next(next);
		} else {
			self.count -= 1;
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
