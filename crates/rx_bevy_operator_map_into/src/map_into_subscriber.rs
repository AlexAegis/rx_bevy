use std::marker::PhantomData;

use rx_bevy_core::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown,
	Tick, Tickable, context::WithSubscriptionContext, prelude::SubscriptionContext,
};

pub struct MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Destination: Subscriber,
{
	destination: Destination,
	_phantom_data: PhantomData<(In, InError, Out, OutError)>,
}

impl<In, InError, Out, OutError, Destination>
	MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Out, OutError, Destination> WithSubscriptionContext
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Out, OutError, Destination> Observer
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
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
		self.destination.next(next.into(), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		self.destination.error(error.into(), context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Out, OutError, Destination> Tickable
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
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

impl<In, InError, Out, OutError, Destination> SubscriptionLike
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
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

impl<In, InError, Out, OutError, Destination> ObserverInput
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Out, OutError, Destination> ObservableOutput
	for MapIntoSubscriber<In, InError, Out, OutError, Destination>
where
	In: SignalBound + Into<Out>,
	InError: SignalBound + Into<OutError>,
	Out: SignalBound,
	OutError: SignalBound,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = OutError;
}
