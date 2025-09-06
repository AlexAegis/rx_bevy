use std::marker::PhantomData;

use rx_bevy_core::{
	Observer, ObserverInput, SignalContext, SubscriptionLike, Tick, UpgradeableObserver,
};
use rx_bevy_subscriber_observer::ObserverSubscriber;

/// An [FnObserver] requires you to define a callback for all three notifications
pub struct FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	on_next: OnPush,
	on_error: OnError,
	on_complete: OnComplete,
	closed: bool,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnPush, OnError, OnComplete> ObserverInput
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
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
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	type Context = Context;
}

impl<In, InError, OnPush, OnError, OnComplete> Observer
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
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

impl<In, InError, OnPush, OnError, OnComplete> SubscriptionLike
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		self.closed = true;
	}
}

impl<In, InError, OnPush, OnError, OnComplete> FnObserver<In, InError, OnPush, OnError, OnComplete>
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
