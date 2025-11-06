use core::marker::PhantomData;

use rx_core_traits::{
	DetachedSubscriber, Observer, ObserverInput, PrimaryCategoryObserver, SignalBound,
	SubscriptionContext, Tick, Tickable, UpgradeableObserver, WithPrimaryCategory,
	WithSubscriptionContext,
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
	_phantom_data: PhantomData<(In, InError, fn(Context))>,
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

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> WithPrimaryCategory
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
	type PrimaryCategory = PrimaryCategoryObserver;
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> UpgradeableObserver
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
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
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
