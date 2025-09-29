use std::marker::PhantomData;

use derive_where::derive_where;
use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber, SubscriptionLike,
	Teardown, Tick,
};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	destination: Destination,
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out, Destination> MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, mapper: Mapper) -> Self {
		Self {
			destination,
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Destination> SignalContext
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Mapper, Out, Destination> Observer
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		let mapped = (self.mapper)(next);
		self.destination.next(mapped, context);
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

impl<In, InError, Mapper, Out, Destination> SubscriptionLike
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.destination.get_unsubscribe_context()
	}
}

impl<In, InError, Mapper, Out, Destination> ObserverInput
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out, Destination> ObservableOutput
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = InError;
}
