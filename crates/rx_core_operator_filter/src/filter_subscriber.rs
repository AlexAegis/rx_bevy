use rx_core_common::{RxObserver, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In, usize) -> bool,
	Destination: Subscriber,
{
	#[destination]
	destination: Destination,
	filter: Filter,
	index: usize,
}

impl<Filter, Destination> FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In, usize) -> bool,
	Destination: Subscriber,
{
	pub fn new(destination: Destination, filter: Filter) -> Self {
		Self {
			destination,
			filter,
			index: 0,
		}
	}
}

impl<Filter, Destination> RxObserver for FilterSubscriber<Filter, Destination>
where
	Filter: for<'a> Fn(&'a Destination::In, usize) -> bool + Send + Sync,
	Destination: Subscriber,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if (self.filter)(&next, self.index) {
			self.destination.next(next);
		}
		self.index = self.index.saturating_add(1)
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
