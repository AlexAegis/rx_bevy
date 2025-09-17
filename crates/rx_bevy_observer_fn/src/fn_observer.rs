use std::marker::PhantomData;

use rx_bevy_core::{DropContext, Observer, ObserverInput, SignalContext, SubscriptionLike, Tick};

/// An [FnObserver] requires you to define a callback for all three notifications
pub struct FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	on_next: OnPush,
	on_error: OnError,
	on_complete: OnComplete,
	closed: bool,
	_phantom_data: PhantomData<(In, InError, Context)>,
}

impl<In, InError, OnPush, OnError, OnComplete, Context> ObserverInput
	for FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnPush, OnError, OnComplete, Context> SignalContext
	for FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, OnPush, OnError, OnComplete, Context> Observer
	for FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
	Context: DropContext,
{
	fn next(&mut self, next: In, _context: &mut Self::Context) {
		(self.on_next)(next);
	}

	fn error(&mut self, error: InError, _context: &mut Self::Context) {
		(self.on_error)(error);
	}

	fn complete(&mut self, _context: &mut Self::Context) {
		(self.on_complete)();
	}

	fn tick(&mut self, _tick: Tick, _context: &mut Self::Context) {}
}

impl<In, InError, OnPush, OnError, OnComplete, Context> SubscriptionLike
	for FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
	Context: DropContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		self.closed = true;
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<In, InError, OnPush, OnError, OnComplete, Context>
	FnObserver<In, InError, OnPush, OnError, OnComplete, Context>
where
	OnPush: Fn(In),
	OnError: Fn(InError),
	OnComplete: Fn(),
{
	pub fn new(next: OnPush, error: OnError, complete: OnComplete) -> Self {
		Self {
			on_next: next,
			on_error: error,
			on_complete: complete,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
