use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

pub struct LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
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
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
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

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> SignalContext
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
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
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		match next {
			Ok(next) => self.destination.next(next, context),
			Err(error) => self.destination.error(error, context),
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination
			.error((self.in_error_to_result_error)(error), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> SubscriptionLike
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
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
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}
impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> SubscriptionCollection
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	Destination: SubscriptionCollection,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.destination.add(subscription, context);
	}
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> ObserverInput
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber,
{
	type In = Result<ResultIn, ResultInError>;
	type InError = InError;
}

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> ObservableOutput
	for LiftResultSubscriber<ResultIn, ResultInError, InError, InErrorToResultError, Destination>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InError: 'static,
	InErrorToResultError: Fn(InError) -> ResultInError,
	Destination: Subscriber,
{
	type Out = ResultIn;
	type OutError = ResultInError;
}
