use core::marker::PhantomData;

use derive_where::derive_where;

use rx_core_common::{PhantomInvariant, RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct CountSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = usize, InError = InError>,
{
	#[destination]
	destination: Destination,
	count: usize,
	_phantom_data: PhantomInvariant<In>,
}

impl<In, InError, Destination> CountSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = usize, InError = InError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			count: 0,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> RxObserver for CountSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = usize, InError = InError>,
{
	#[inline]
	fn next(&mut self, _next: Self::In) {
		self.count += 1;
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(self.count);
		self.destination.complete();
	}
}
