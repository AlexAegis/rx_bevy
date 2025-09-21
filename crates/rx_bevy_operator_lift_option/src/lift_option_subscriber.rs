use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

pub struct LiftOptionSubscriber<In, InError, Destination>
where
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Destination> LiftOptionSubscriber<In, InError, Destination>
where
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
	In: 'static,
	InError: 'static,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Destination> SignalContext for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Destination> Observer for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		if let Some(next) = next {
			self.destination.next(next, context);
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

	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Destination> SubscriptionLike for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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

impl<In, InError, Destination> SubscriptionCollection
	for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
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

impl<In, InError, Destination> ObserverInput for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type In = Option<In>;
	type InError = InError;
}

impl<In, InError, Destination> ObservableOutput for LiftOptionSubscriber<In, InError, Destination>
where
	In: 'static,
	InError: 'static,
	Destination: Subscriber,
{
	type Out = In;
	type OutError = InError;
}
