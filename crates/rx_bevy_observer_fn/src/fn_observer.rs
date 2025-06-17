use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverInput};

pub struct ObserverCallbacks<OnPush, OnError, OnComplete> {
	next: OnPush,
	error: OnError,
	complete: OnComplete,
}

/// A simple observer that prints out received values using [std::fmt::Debug]
///
pub struct FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
{
	next: OnPush,
	error: OnError,
	complete: OnComplete,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnPush, OnError, OnComplete> ObserverInput
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, OnPush, OnError, OnComplete> Observer
	for FnObserver<In, InError, OnPush, OnError, OnComplete>
where
	OnPush: FnMut(In),
	OnError: FnMut(InError),
	OnComplete: FnMut(),
	In: 'static,
	InError: 'static,
{
	fn next(&mut self, next: In) {
		(self.next)(next);
	}

	fn error(&mut self, error: InError) {
		(self.error)(error);
	}

	fn complete(&mut self) {
		(self.complete)();
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
			next,
			error,
			complete,
			_phantom_data: PhantomData,
		}
	}

	pub fn new_from(observer_callbacks: ObserverCallbacks<OnPush, OnError, OnComplete>) -> Self {
		Self {
			next: observer_callbacks.next,
			error: observer_callbacks.error,
			complete: observer_callbacks.complete,
			_phantom_data: PhantomData,
		}
	}
}
