use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber, SubscriptionContext};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
#[rx_delegate_tickable_to_destination]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In) -> bool,
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	filter: Filter,
}

impl<Filter, Destination> FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In) -> bool,
	Destination: Subscriber,
{
	pub fn new(destination: Destination, filter: Filter) -> Self {
		Self {
			destination,
			filter,
		}
	}
}

impl<Filter, Destination> Observer for FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In) -> bool + Send + Sync,
	Destination: Subscriber,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if (self.filter)(&next) {
			self.destination.next(next, context);
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
