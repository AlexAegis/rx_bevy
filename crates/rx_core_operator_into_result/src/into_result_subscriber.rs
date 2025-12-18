use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

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
	_phantom_data: PhantomData<(In, InError)>,
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

impl<In, InError, Destination> Observer for IntoResultSubscriber<In, InError, Destination>
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
