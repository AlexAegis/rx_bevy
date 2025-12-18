use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
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
	fn next(&mut self, next: Self::In) {
		if (self.filter)(&next) {
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
