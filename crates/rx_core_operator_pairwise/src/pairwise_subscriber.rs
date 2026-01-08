use derive_where::derive_where;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive_where(Debug; In)]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct PairwiseSubscriber<In, Destination>
where
	In: Signal + Clone,
	Destination: Subscriber<In = [In; 2]>,
{
	#[destination]
	#[derive_where(skip)]
	destination: Destination,
	last: Option<In>,
}

impl<In, Destination> PairwiseSubscriber<In, Destination>
where
	In: Signal + Clone,
	Destination: Subscriber<In = [In; 2]>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			last: None,
		}
	}
}

impl<In, Destination> RxObserver for PairwiseSubscriber<In, Destination>
where
	In: Signal + Clone,
	Destination: Subscriber<In = [In; 2]>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if let Some(last) = self.last.replace(next.clone()) {
			self.destination.next([last, next]);
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
