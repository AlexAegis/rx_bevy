use core::marker::PhantomData;

use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, SignalBound, SubscriptionContext, Tick, Tickable};

/// An [FnObserver] requires you to define a callback for all three notifications
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(Context)]
pub struct FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: 'static + FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: 'static + FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: 'static + FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	on_next: OnNext,
	on_error: OnError,
	on_complete: OnComplete,
	on_tick: OnTick,
	_phantom_data: PhantomData<(In, InError, fn(Context))>,
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
	FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: 'static + FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: 'static + FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: 'static + FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
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
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> Observer
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: 'static + FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: 'static + FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: 'static + FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		next: In,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		(self.on_next)(next, context);
	}

	fn error(
		&mut self,
		error: InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		(self.on_error)(error, context);
	}

	fn complete(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		(self.on_complete)(context);
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> Tickable
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + FnMut(In, &mut Context::Item<'_, '_>) + Send + Sync,
	OnError: 'static + FnMut(InError, &mut Context::Item<'_, '_>) + Send + Sync,
	OnComplete: 'static + FnMut(&mut Context::Item<'_, '_>) + Send + Sync,
	OnTick: 'static + FnMut(Tick, &mut Context::Item<'_, '_>) + Send + Sync,
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
