use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

#[derive(RxSubscriber, Debug)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + FnMut(&In),
	Destination: Subscriber<In = In, InError = InError>,
	In: Signal,
	InError: Signal,
{
	#[destination]
	destination: Destination,
	callback: OnNext,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, Destination> TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + FnMut(&In),
	Destination: Subscriber<In = In, InError = InError>,
	In: Signal,
	InError: Signal,
{
	pub fn new(destination: Destination, callback: OnNext) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Destination> Observer
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + FnMut(&In) + Send + Sync,
	Destination: Subscriber<In = In, InError = InError>,
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		(self.callback)(&next);
		self.destination.next(next);
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
