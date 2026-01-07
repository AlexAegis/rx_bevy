use core::marker::PhantomData;

use rx_core_common::{Observer, Signal};
use rx_core_macro_observer_derive::RxObserver;

/// An [FnObserver] requires you to define a callback for all three notifications
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct FnObserver<In, InError, OnNext, OnError, OnComplete>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(In) + Send + Sync,
	OnError: 'static + FnOnce(InError) + Send + Sync,
	OnComplete: 'static + FnOnce() + Send + Sync,
{
	on_next: OnNext,
	on_error: Option<OnError>,
	on_complete: Option<OnComplete>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext, OnError, OnComplete> FnObserver<In, InError, OnNext, OnError, OnComplete>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(In) + Send + Sync,
	OnError: 'static + FnOnce(InError) + Send + Sync,
	OnComplete: 'static + FnOnce() + Send + Sync,
{
	pub fn new(on_next: OnNext, on_error: OnError, on_complete: OnComplete) -> Self {
		Self {
			on_next,
			on_error: Some(on_error),
			on_complete: Some(on_complete),
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
	OnError: 'static + FnOnce(InError) + Send + Sync,
	OnComplete: 'static + FnOnce() + Send + Sync,
{
	fn next(&mut self, next: In) {
		(self.on_next)(next);
	}

	fn error(&mut self, error: InError) {
		if let Some(on_error) = self.on_error.take() {
			(on_error)(error);
		}
	}

	fn complete(&mut self) {
		if let Some(on_complete) = self.on_complete.take() {
			(on_complete)();
		}
	}
}
