use std::marker::PhantomData;

use rx_core_traits::{
	ObservableOutput, Observer, ObserverInput, SignalBound, Subscriber, SubscriptionLike, Teardown,
	Tick, Tickable,
	SubscriptionContext, WithSubscriptionContext,
};

pub struct TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	destination: Destination,
	callback: OnNext,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, Destination> TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	pub fn new(destination: Destination, callback: OnNext) -> Self {
		Self {
			destination,
			callback,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, Destination> WithSubscriptionContext
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type Context = Destination::Context;
}

impl<In, InError, OnNext, Destination> Observer
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		(self.callback)(&next, context);
		self.destination.next(next, context);
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

impl<In, InError, OnNext, Destination> Tickable
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
	Destination: SubscriptionLike,
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

impl<In, InError, OnNext, Destination> SubscriptionLike
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
	Destination: SubscriptionLike,
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

impl<In, InError, OnNext, Destination> ObservableOutput
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError, OnNext, Destination> ObserverInput
	for TapNextSubscriber<In, InError, OnNext, Destination>
where
	OnNext: 'static + Fn(&In, &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>),
	Destination: Subscriber<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}
