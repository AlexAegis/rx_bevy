use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, Signal, Subscriber};

use crate::LiftResultSubscriber;

/// The [LiftResultOperator] unwraps a Result and passes its Ok(T) variant, and
/// errors its Err(E) variant downstream. It also requires a mapping function
/// to normalize the upstream error to the new downstream error type, defined
/// by the results Err variant.
///
/// The reason it's not called an "UnwrapResultOperator" is that would imply
/// it can panic; that is only true if the error isn't caught downstream.
#[derive_where(Default)]
#[derive(RxOperator)]
#[rx_in(Result<ResultIn, ResultInError>)]
#[rx_in_error(Never)]
#[rx_out(ResultIn)]
#[rx_out_error(ResultInError)]
pub struct LiftResultOperator<ResultIn, ResultInError, InError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError)>,
}

impl<ResultIn, ResultInError, InError> ComposableOperator
	for LiftResultOperator<ResultIn, ResultInError, InError>
where
	ResultIn: Signal,
	ResultInError: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= LiftResultSubscriber<ResultIn, ResultInError, InError, Destination>
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
		LiftResultSubscriber::new(destination)
	}
}
