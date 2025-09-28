use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, InnerSubscription, Observer, ObserverInput, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown, Tick,
};

/// An [FnObserver] requires you to define a callback for all three notifications
pub struct FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	on_next: OnNext,
	on_error: OnError,
	on_complete: OnComplete,
	on_tick: OnTick,
	teardown: InnerSubscription<Context>,
	_phantom_data: PhantomData<*mut (In, InError)>,
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
	FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
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
			teardown: InnerSubscription::default(),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> ObserverInput
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> SignalContext
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> Observer
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	fn next(&mut self, next: In, context: &mut Self::Context) {
		if !self.is_closed() {
			(self.on_next)(next, context);
		}
	}

	fn error(&mut self, error: InError, context: &mut Self::Context) {
		if !self.is_closed() {
			(self.on_error)(error, context);
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			(self.on_complete)(context);
			self.unsubscribe(context);
		}
	}

	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if !self.is_closed() {
			(self.on_tick)(tick, context);
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> SubscriptionLike
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.teardown.unsubscribe(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<In, InError, OnNext, OnError, OnComplete, OnTick, Context> SubscriptionCollection
	for FnObserver<In, InError, OnNext, OnError, OnComplete, OnTick, Context>
where
	In: 'static,
	InError: 'static,
	OnNext: FnMut(In, &mut Context),
	OnError: FnMut(InError, &mut Context),
	OnComplete: FnMut(&mut Context),
	OnTick: FnMut(Tick, &mut Context),
	Context: DropContext,
{
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.teardown.add(subscription, context);
	}
}
