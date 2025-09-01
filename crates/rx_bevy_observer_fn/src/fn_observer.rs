use std::marker::PhantomData;

use rx_bevy_core::{Observer, ObserverInput, UpgradeableObserver, prelude::ObserverSubscriber};

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

impl<In, InError, OnPush, OnError, OnComplete> Observer
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	In: 'static,
	InError: 'static,
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	fn next(&mut self, next: In) {
		(self.on_next)(next);
	}

	fn error(&mut self, error: InError) {
		(self.on_error)(error);
	}

	fn complete(&mut self) {
		(self.on_complete)();
	}

	#[cfg(feature = "tick")]
	fn tick(&mut self, _tick: rx_bevy_core::Tick) {}
}

impl<In, InError, OnPush, OnError, OnComplete> UpgradeableObserver
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	In: 'static,
	InError: 'static,
	OnPush: 'static + FnMut(In),
	OnError: 'static + FnMut(InError),
	OnComplete: 'static + FnMut(),
{
	type Subscriber = ObserverSubscriber<Self>;

	fn upgrade(self) -> Self::Subscriber {
		ObserverSubscriber::new(self)
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
			_phantom_data: PhantomData,
		}
	}
}
