use std::marker::PhantomData;

use rx_core_traits::{
	Observer, ObserverInput, SignalBound, SubscriptionContext, SubscriptionData, SubscriptionLike,
	Teardown, Tick, Tickable, WithSubscriptionContext,
};

/// An [FnObserver] requires you to define a callback for all three notifications
pub struct FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	on_next: OnNext,
	on_error: OnError,
	on_complete: OnComplete,
	on_tick: OnTick,
	teardown: SubscriptionData<Context>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
	FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	pub fn new(
		on_next: OnNext,
		on_error: OnError,
		on_complete: OnComplete,
		on_tick: OnTick,
	) -> Self {
		Self {
			on_next,
			on_error,
			on_complete,
			on_tick,
			teardown: SubscriptionData::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> ObserverInput
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> WithSubscriptionContext
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> Observer
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			(self.on_next)(next, context);
		}
	}

	fn error(
		&mut self,
		error: InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			(self.on_error)(error, context);
		}
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			(self.on_complete)(context);
			self.unsubscribe(context);
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> Tickable
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		(self.on_tick)(tick, context);
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> SubscriptionLike
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.teardown.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}
