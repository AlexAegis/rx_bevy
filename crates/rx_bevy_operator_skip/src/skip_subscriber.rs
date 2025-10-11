use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown,
	Tick, Tickable, WithContext,
};

pub struct SkipSubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	count: usize,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, count: usize) -> Self {
		Self {
			destination,
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> WithContext for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Destination> Observer for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if self.count == 0 {
			self.destination.next(next, context);
		} else {
			self.count -= 1;
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut Self::Context) {
		self.destination.complete(context);
	}
}

impl<In, InError, Destination> Tickable for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Destination> SubscriptionLike for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
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
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.destination.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<In, InError, Destination> ObservableOutput for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, Destination> ObserverInput for SkipSubscriber<In, InError, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}
