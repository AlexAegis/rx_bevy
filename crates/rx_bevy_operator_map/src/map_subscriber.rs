use std::marker::PhantomData;

use derive_where::derive_where;
use rx_bevy_core::{
	SubscriptionCollection, ObservableOutput, Observer, ObserverInput, Operation,
	SignalContext, Subscriber, SubscriptionLike,
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
}

impl<In, InError, Mapper, Out, Destination> SubscriptionCollection
	for MapSubscriber<In, InError, Mapper, Out, Destination>
where
	In: 'static,
	InError: 'static,
	Mapper: Fn(In) -> Out,
	Out: 'static,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		> + SubscriptionCollection,
{
	#[inline]
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	) {
		self.destination.add(subscription, context);
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

impl<In, InError, Mapper, Out, Destination> Operation
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
