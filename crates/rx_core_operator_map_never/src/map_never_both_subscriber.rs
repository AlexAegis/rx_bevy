use rx_core_common::{Never, Observer, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Never)]
#[rx_in_error(Never)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct MapNeverBothSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
}

impl<Destination> MapNeverBothSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> Observer for MapNeverBothSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, _next: Self::In) {
		unreachable!("In is Never");
	}

	#[inline]
	fn error(&mut self, _error: Self::InError) {
		unreachable!("InError is Never");
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
