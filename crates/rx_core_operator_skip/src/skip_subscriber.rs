use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionContext};

#[derive(RxSubscriber)]
#[rx_context(Destination::Context)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection_to_destination]
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
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if self.count == 0 {
			self.destination.next(next, context);
		} else {
			self.count -= 1;
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}
