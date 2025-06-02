use std::marker::PhantomData;

use rx_bevy_observable::Observer;

pub struct ObserverCallbacks<OnPush, OnError, OnComplete> {
	next: OnPush,
	error: OnError,
	complete: OnComplete,
}

/// A simple observer that prints out received values using [std::fmt::Debug]
///
pub struct FnObserver<In, Error, OnPush, OnError, OnComplete>
where
	OnPush: Fn(In) -> (),
	OnError: Fn(Error) -> (),
	OnComplete: Fn() -> (),
{
	next: OnPush,
	error: OnError,
	complete: OnComplete,
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Error, OnPush, OnError, OnComplete> Observer
	for FnObserver<In, Error, OnPush, OnError, OnComplete>
where
	OnPush: Fn(In) -> (),
	OnError: Fn(Error) -> (),
	OnComplete: Fn() -> (),
{
	type In = In;
	type Error = Error;

	fn next(&mut self, next: In) {
		(self.next)(next);
	}

	fn error(&mut self, error: Error) {
		(self.error)(error);
	}

	fn complete(&mut self) {
		(self.complete)();
	}
}

impl<In, Error, OnPush, OnError, OnComplete> FnObserver<In, Error, OnPush, OnError, OnComplete>
where
	OnPush: Fn(In) -> (),
	OnError: Fn(Error) -> (),
	OnComplete: Fn() -> (),
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
