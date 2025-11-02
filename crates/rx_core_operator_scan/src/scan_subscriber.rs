use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_traits::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionContext,
	SubscriptionLike, Teardown, Tick, Tickable, WithSubscriptionContext,
};

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber,
{
	destination: Destination,
	accumulator: Out,
	reducer: Reducer,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Reducer, Out, Destination> ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	pub fn new(destination: Destination, reducer: Reducer, seed: Out) -> Self {
		Self {
			accumulator: seed,
			destination,
			reducer,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Reducer, Out, Destination> WithSubscriptionContext
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	type Context = Destination::Context;
}

impl<In, InError, Reducer, Out, Destination> Observer
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound + Clone,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.accumulator = (self.reducer)(&self.accumulator, next);
		self.destination.next(self.accumulator.clone(), context);
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.complete(context);
	}
}

impl<In, InError, Reducer, Out, Destination> Tickable
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber<
			In = <Self as ObservableOutput>::Out,
			InError = <Self as ObservableOutput>::OutError,
		>,
{
	#[inline]
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.tick(tick, context);
	}
}

impl<In, InError, Reducer, Out, Destination> SubscriptionLike
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
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
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<In, InError, Reducer, Out, Destination> ObserverInput
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Reducer, Out, Destination> ObservableOutput
	for ScanSubscriber<In, InError, Reducer, Out, Destination>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out,
	Out: SignalBound,
	Destination: Subscriber,
{
	type Out = Out;
	type OutError = InError;
}
