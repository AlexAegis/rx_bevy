use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber};

use crate::LiftResultSubscriber;

/// The [LiftResultOperator] unwraps a Result and passes it's Ok(T) variant, and
/// errors it's Err(E) variant downstream. It also requires a mapping function
/// to normalize the upstream error to the new downstream error type, defined
/// by the results Err variant.
///
/// The reason it's not called an "UnwrapResultOperator" because that would imply
/// that it can panic, however that's only true if the error isn't caught downstream.
#[derive(RxOperator)]
#[rx_in(Result<ResultIn, ResultInError>)]
#[rx_in_error(InError)]
#[rx_out(ResultIn)]
#[rx_out_error(ResultInError)]
pub struct LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	in_error_to_result_error: InErrorToResultError,
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError, InErrorToResultError)>,
}

impl<ResultIn, ResultInError, InError, InErrorToResultError>
	LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	pub fn new(in_error_to_result_error: InErrorToResultError) -> Self {
		Self {
			in_error_to_result_error,
			_phantom_data: PhantomData,
		}
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError> Operator
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: 'static + Fn(InError) -> ResultInError + Clone + Send + Sync,
{
	type Subscriber<Destination>
		= LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		LiftResultSubscriber::new(destination, self.in_error_to_result_error.clone())
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError> Clone
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	fn clone(&self) -> Self {
		Self {
			in_error_to_result_error: self.in_error_to_result_error.clone(),
			_phantom_data: PhantomData,
		}
	}
}
