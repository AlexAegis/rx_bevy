use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown,
	Tick, Tickable, context::WithSubscriptionContext, prelude::SubscriptionContext,
};

pub struct LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber,
{
	destination: Destination,
	in_error_to_result_error: InErrorToResultError,
	_phantom_data: PhantomData<(ResultIn, ResultInError, InError, InErrorToResultError)>,
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
	LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, in_error_to_result_error: InErrorToResultError) -> Self {
		Self {
			destination,
			in_error_to_result_error,
			_phantom_data: PhantomData,
		}
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> WithSubscriptionContext
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> Observer
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		match next {
			Ok(next) => self.destination.next(next, context),
			Err(error) => {
				self.destination.error(error, context);
				self.destination.unsubscribe(context);
			}
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		self.destination
			.error((self.in_error_to_result_error)(error), context);
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.destination.complete(context);
		self.destination.unsubscribe(context);
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> Tickable
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.destination.tick(tick, context);
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> SubscriptionLike
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> ObserverInput
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber,
{
	type In = Result<ResultIn, ResultInError>;
	type InError = InError;
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> ObservableOutput
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber,
{
	type Out = ResultIn;
	type OutError = ResultInError;
}
