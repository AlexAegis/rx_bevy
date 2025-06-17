use rx_bevy_observable::{Observer, ObserverInput};

/// A simple observer that prints out received values using [std::fmt::Debug]
pub struct DynFnObserver<In, Error> {
	on_next: Option<Box<dyn FnMut(In)>>,
	on_error: Option<Box<dyn FnMut(Error)>>,
	on_complete: Option<Box<dyn FnMut()>>,
}

impl<In, InError> ObserverInput for DynFnObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for DynFnObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn next(&mut self, next: In) {
		if let Some(on_next) = &mut self.on_next {
			(on_next)(next);
		}
	}

	fn error(&mut self, error: InError) {
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

impl<In, InError> Default for DynFnObserver<In, InError> {
	fn default() -> Self {
		Self {
			on_next: None,
			on_error: None,
			on_complete: None,
		}
	}
}

impl<In, InError> DynFnObserver<In, InError> {
	pub fn with_next<OnPush: 'static + FnMut(In)>(self, next: OnPush) -> Self {
		Self {
			on_next: Some(Box::new(next)),
			..self
		}
	}

	pub fn with_error<OnError: 'static + FnMut(InError)>(self, error: OnError) -> Self {
		Self {
			on_error: Some(Box::new(error)),
			..self
		}
	}

	pub fn with_complete<OnComplete: 'static + FnMut()>(self, complete: OnComplete) -> Self {
		Self {
			on_complete: Some(Box::new(complete)),
			..self
		}
	}
}
