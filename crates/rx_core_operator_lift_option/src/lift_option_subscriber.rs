use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(Option<Destination::In>)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct LiftOptionSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
}

impl<Destination> LiftOptionSubscriber<Destination>
where
	Destination: Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self { destination }
	}
}

impl<Destination> Observer for LiftOptionSubscriber<Destination>
where
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if let Some(next) = next {
			self.destination.next(next);
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
