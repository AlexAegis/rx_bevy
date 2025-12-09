use core::marker::PhantomData;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{Observer, Signal, Subscriber};

#[derive(RxSubscriber)]
#[rx_in(Result<ResultIn, ResultInError>)]
#[rx_in_error(InError)]
#[rx_delegate_teardown_collection_to_destination]
#[rx_delegate_subscription_like_to_destination]
pub struct LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Fn(InError) -> ResultInError + Send + Sync,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	#[destination]
	destination: Destination,
	in_error_to_result_error: InErrorToResultError,
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError, InErrorToResultError)>,
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
	LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Fn(InError) -> ResultInError + Send + Sync,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	pub fn new(destination: Destination, in_error_to_result_error: InErrorToResultError) -> Self {
		Self {
			destination,
			in_error_to_result_error,
			_phantom_data: PhantomData,
		}
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> Observer
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Fn(InError) -> ResultInError + Send + Sync,
	Destination: Subscriber<In = ResultIn, InError = ResultInError>,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		match next {
			Ok(next) => self.destination.next(next),
			Err(error) => {
				self.destination.error(error);
				self.destination.unsubscribe();
			}
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination
			.error((self.in_error_to_result_error)(error));
	}

	#[inline]
	fn complete(&mut self) {
		self.destination.complete();
	}
}
