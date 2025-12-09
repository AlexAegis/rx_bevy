use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct EnumerateSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = (In, usize)>,
{
	#[destination]
	destination: Destination,
	counter: usize,
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
			counter: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Destination> Observer for EnumerateSubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = (In, usize)>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next((next, self.counter));

		// Increment after emission, so the first value could be 0
		#[cfg(feature = "saturating_add")]
		{
			self.counter = self.counter.saturating_add(1);
		}
		#[cfg(not(feature = "saturating_add"))]
		{
			self.counter += 1;
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
