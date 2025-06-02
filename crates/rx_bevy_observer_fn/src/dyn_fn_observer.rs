use std::marker::PhantomData;

use rx_bevy_observable::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error> {
	on_push: Option<Box<dyn Fn(In) -> ()>>,
	on_error: Option<Box<dyn Fn(Error) -> ()>>,
	on_complete: Option<Box<dyn Fn() -> ()>>,
}

impl<In, Error> Observer for DynFnObserver<In, Error> {
	type In = In;
	type Error = Error;

	fn on_push(&mut self, value: In) {
		if let Some(on_push) = &self.on_push {
			(on_push)(value);
		}
	}

	fn on_error(&mut self, error: Error) {
		if let Some(on_error) = &self.on_error {
			(on_error)(error);
		}
	}

	fn on_complete(&mut self) {
		if let Some(on_complete) = &self.on_complete {
			(on_complete)();
		}
	}
}

impl<In, Error> Default for DynFnObserver<In, Error> {
	fn default() -> Self {
		Self {
			on_push: None,
			on_error: None,
			on_complete: None,
		}
	}
}

impl<In, Error> DynFnObserver<In, Error> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_on_push<OnPush: 'static + Fn(In) -> ()>(self, on_push: OnPush) -> Self {
		Self {
			on_push: Some(Box::new(on_push)),
			..self
		}
	}

	pub fn with_on_error<OnError: 'static + Fn(Error) -> ()>(self, on_error: OnError) -> Self {
		Self {
			on_error: Some(Box::new(on_error)),
			..self
		}
	}

	pub fn with_on_complete<OnComplete: 'static + Fn() -> ()>(
		self,
		on_complete: OnComplete,
	) -> Self {
		Self {
			on_complete: Some(Box::new(on_complete)),
			..self
		}
	}
}
