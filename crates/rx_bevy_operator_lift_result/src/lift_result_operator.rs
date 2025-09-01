use std::marker::PhantomData;

use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, Subscriber};

use crate::LiftResultSubscriber;

/// The [LiftResultOperator] unwraps a Result and passes it's Ok(T) variant, and
/// errors it's Err(E) variant downstream. It also requires a mapping function
/// to normalize the upstream error to the new downstream error type, defined
/// by the results Err variant.
///
/// The reason it's not called an "UnwrapResultOperator" because that would imply
/// that it can panic, however that's only true if the error isn't caught downstream.
pub struct LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	InError: 'static,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	in_error_to_result_error: InErrorToResultError,
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError, InErrorToResultError)>,
}

impl<ResultIn, ResultInError, InError, InErrorToResultError>
	LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
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
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: 'static + Clone + Fn(InError) -> ResultInError,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>;

	fn operator_subscribe<
		Destination: 'static
			+ Subscriber<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		LiftResultSubscriber::new(destination, self.in_error_to_result_error.clone())
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError> ObserverInput
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	type In = Result<ResultIn, ResultInError>;
	type InError = InError;
}

impl<ResultIn, ResultInError, InError, InErrorToResultError> ObservableOutput
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	type Out = ResultIn;
	type OutError = ResultInError;
}

impl<ResultIn, ResultInError, InError, InErrorToResultError> Clone
	for LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	fn clone(&self) -> Self {
		Self {
			in_error_to_result_error: self.in_error_to_result_error.clone(),
			_phantom_data: PhantomData,
		}
	}
}
