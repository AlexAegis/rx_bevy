use std::marker::PhantomData;

use rx_bevy_observable::Observer;

pub struct ObserverCallbacks<OnPush, OnError, OnComplete> {
	on_push: OnPush,
	on_error: OnError,
	on_complete: OnComplete,
}

/// A simple observer that prints out received values using [std::fmt::Debug]
///
pub struct FnObserver<In, Error, OnPush, OnError, OnComplete>
where
	OnPush: Fn(In) -> (),
	OnError: Fn(Error) -> (),
	OnComplete: Fn() -> (),
{
	on_push: OnPush,
	on_error: OnError,
	on_complete: OnComplete,
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

	fn on_push(&mut self, value: In) {
		(self.on_push)(value);
	}

	fn on_error(&mut self, error: Error) {
		(self.on_error)(error);
	}

	fn on_complete(&mut self) {
		(self.on_complete)();
	}
}

impl<In, Error, OnPush, OnError, OnComplete> FnObserver<In, Error, OnPush, OnError, OnComplete>
where
	OnPush: Fn(In) -> (),
	OnError: Fn(Error) -> (),
	OnComplete: Fn() -> (),
{
	pub fn new(on_push: OnPush, on_error: OnError, on_complete: OnComplete) -> Self {
		Self {
			on_push,
			on_error,
			on_complete,
			_phantom_data: PhantomData,
		}
	}

	pub fn new_from(observer_callbacks: ObserverCallbacks<OnPush, OnError, OnComplete>) -> Self {
		Self {
			on_push: observer_callbacks.on_push,
			on_error: observer_callbacks.on_error,
			on_complete: observer_callbacks.on_complete,
			_phantom_data: PhantomData,
		}
	}
}
