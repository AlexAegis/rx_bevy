use core::marker::PhantomData;

use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, Signal};

/// An [FnObserver] requires you to define a callback for all three notifications
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct FnObserver<In, InError, OnNext, OnError, OnComplete>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(In) + Send + Sync,
	OnError: 'static + FnMut(InError) + Send + Sync,
	OnComplete: 'static + FnMut() + Send + Sync,
{
	on_next: OnNext,
	on_error: OnError,
	on_complete: OnComplete,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, OnError, OnComplete> FnObserver<In, InError, OnNext, OnError, OnComplete>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(In) + Send + Sync,
	OnError: 'static + FnMut(InError) + Send + Sync,
	OnComplete: 'static + FnMut() + Send + Sync,
{
	pub fn new(on_next: OnNext, on_error: OnError, on_complete: OnComplete) -> Self {
		Self {
			on_next,
			on_error,
			on_complete,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext, OnError, OnComplete> Observer
	for FnObserver<In, InError, OnNext, OnError, OnComplete>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(In) + Send + Sync,
	OnError: 'static + FnMut(InError) + Send + Sync,
	OnComplete: 'static + FnMut() + Send + Sync,
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
}
