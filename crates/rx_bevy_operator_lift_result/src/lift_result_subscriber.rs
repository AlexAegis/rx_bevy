use std::marker::PhantomData;

use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Subscriber, SubscriptionLike,
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
	fn next(&mut self, next: Self::In) {
		match next {
			Ok(next) => self.destination.next(next),
			Err(error) => self.destination.error(error),
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

	#[cfg(feature = "tick")]
	#[inline]
	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		self.destination.tick(tick);
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
	fn unsubscribe(&mut self) {
		self.destination.unsubscribe();
	}

	#[inline]
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.destination.add(subscription);
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
