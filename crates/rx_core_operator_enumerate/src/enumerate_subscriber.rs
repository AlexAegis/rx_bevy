use core::marker::PhantomData;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct EnumerateSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = (In, usize)>,
{
	#[destination]
	destination: Destination,
	index: usize,
	_phantom_data: PhantomData<In>,
}

impl<In, Destination> EnumerateSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = (In, usize)>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			index: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Destination> RxObserver for EnumerateSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = (In, usize)>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next((next, self.index));
		// Increment after emission, so the first index is 0
		self.index = self.index.saturating_add(1);
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
