use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, Operation, SignalContext, Subscriber,
	SubscriptionCollection, SubscriptionLike, Teardown, Tick,
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
	fn next<'c>(&mut self, next: Self::In, context: &mut Self::Context<'c>) {
		match next {
			Ok(next) => self.destination.next(next, context),
			Err(error) => self.destination.error(error, context),
		}
	}

	#[inline]
	fn error<'c>(&mut self, error: Self::InError, context: &mut Self::Context<'c>) {
		self.destination
			.error((self.in_error_to_result_error)(error), context);
	}

	#[inline]
	fn complete<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.complete(context);
	}

	#[inline]
	fn tick<'c>(&mut self, tick: Tick, context: &mut Self::Context<'c>) {
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
	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>) {
		self.destination.unsubscribe(context);
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
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
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

impl<ResultIn, ResultInError, InError, InErrorToResultError, Destination> Operation
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
	type Destination = Destination;

	#[inline]
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		reader(&self.destination);
	}

	#[inline]
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		writer(&mut self.destination);
	}
}
