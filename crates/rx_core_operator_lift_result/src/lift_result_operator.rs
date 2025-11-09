use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, SignalBound, Subscriber, SubscriptionContext};

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
#[rx_context(Context)]
pub struct LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError, Context = ()>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
	Context: SubscriptionContext,
{
	in_error_to_result_error: InErrorToResultError,
	_phantom_data: PhantomData<(
		ResultIn,
		ResultInError,
		InError,
		InErrorToResultError,
		Context,
	)>,
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Context>
	LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError, Context>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
	Context: SubscriptionContext,
{
	pub fn new(in_error_to_result_error: InErrorToResultError) -> Self {
		Self {
			in_error_to_result_error,
			_phantom_data: PhantomData,
		}
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Context> Operator
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError, Context>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: 'static + Fn(InError) -> ResultInError + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	type Subscriber<Destination>
		= LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		LiftResultSubscriber::new(destination, self.in_error_to_result_error.clone())
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Context> Clone
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError, Context>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			in_error_to_result_error: self.in_error_to_result_error.clone(),
			_phantom_data: PhantomData,
		}
	}
}
