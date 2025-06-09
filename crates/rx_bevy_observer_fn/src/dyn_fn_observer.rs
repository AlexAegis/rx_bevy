use rx_bevy_observable::Observer;

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error> {
	on_next: Option<Box<dyn FnMut(In) -> ()>>,
	on_error: Option<Box<dyn FnMut(Error) -> ()>>,
	on_complete: Option<Box<dyn FnMut() -> ()>>,
}

impl<In, Error> Observer for DynFnObserver<In, Error> {
	type In = In;
	type Error = Error;

	fn next(&mut self, next: In) {
		if let Some(on_next) = &mut self.on_next {
			(on_next)(next);
		}
	}

	fn error(&mut self, error: Error) {
		if let Some(on_error) = &mut self.on_error {
			(on_error)(error);
		}
	}

	fn complete(&mut self) {
		if let Some(on_complete) = &mut self.on_complete {
			(on_complete)();
		}
	}
}

impl<In, Error> Default for DynFnObserver<In, Error> {
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
		}
	}
}

impl<In, Error> DynFnObserver<In, Error> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_next<OnPush: 'static + FnMut(In) -> ()>(self, next: OnPush) -> Self {
		Self {
			on_next: Some(Box::new(next)),
			..self
		}
	}

	pub fn with_error<OnError: 'static + FnMut(Error) -> ()>(self, error: OnError) -> Self {
		Self {
			on_error: Some(Box::new(error)),
			..self
		}
	}

	pub fn with_complete<OnComplete: 'static + FnMut() -> ()>(self, complete: OnComplete) -> Self {
		Self {
			on_complete: Some(Box::new(complete)),
			..self
		}
	}
}
