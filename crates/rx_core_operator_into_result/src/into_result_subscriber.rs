use core::marker::PhantomData;

use rx_core_common::{PhantomInvariant, RxObserver, Signal, Subscriber};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct IntoResultSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = Result<In, InError>>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, Destination> IntoResultSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = Result<In, InError>>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> RxObserver for IntoResultSubscriber<In, InError, Destination>
where
	In: Signal,
	InError: Signal,
	Destination: Subscriber<In = Result<In, InError>>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.destination.next(Ok(next));
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.next(Err(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
