use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Never, Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(Result<ResultIn, ResultInError>)]
#[rx_in_error(Never)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct LiftResultSubscriber<ResultIn, ResultInError, InError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	#[destination]
	destination: Destination,
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError)>,
}

impl<ResultIn, ResultInError, InError, Destination>
	LiftResultSubscriber<ResultIn, ResultInError, InError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<ResultIn, ResultInError, InError, Destination> Observer
	for LiftResultSubscriber<ResultIn, ResultInError, InError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		match next {
			Ok(next) => self.destination.next(next),
			Err(error) => {
				self.destination.error(error);
			}
		}
	}

	#[inline]
	fn error(&mut self, _error: Self::InError) {
		unreachable!("InError is of type Never")
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
