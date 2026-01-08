use core::marker::PhantomData;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: Signal + Into<Out>,
	InError: Signal + Into<OutError>,
	Out: Signal,
	OutError: Signal,
	Destination: Subscriber<In = Out, InError = OutError>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Out, OutError, Destination>
	MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: Signal + Into<Out>,
	InError: Signal + Into<OutError>,
	Out: Signal,
	OutError: Signal,
	Destination: Subscriber<In = Out, InError = OutError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, OutError, Destination> RxObserver
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: Signal + Into<Out>,
	InError: Signal + Into<OutError>,
	Out: Signal,
	OutError: Signal,
	Destination: Subscriber<In = Out, InError = OutError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(next.into());
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error.into());
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
