use std::marker::PhantomData;

use rx_core_common::{RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct IsEmptySubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = bool>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<In>,
}

impl<In, Destination> IsEmptySubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = bool>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, Destination> RxObserver for IsEmptySubscriber<In, Destination>
where
	In: Signal,
	Destination: Subscriber<In = bool>,
{
	#[inline]
	fn next(&mut self, _next: Self::In) {
		self.destination.next(false);
		if !self.destination.is_closed() {
			self.destination.complete();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.next(true);
		if !self.destination.is_closed() {
			self.destination.complete();
		}
	}
}
