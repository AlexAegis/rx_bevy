use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverInput};

#[derive(Default, Debug)]
pub struct NoopObserver<In, Error> {
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Error> ObserverInput for NoopObserver<In, Error> {
	type In = In;
	type InError = Error;
}

impl<In, Error> Observer for NoopObserver<In, Error> {
	fn next(&mut self, _next: Self::In) {}

	fn error(&mut self, _error: Self::InError) {}

	fn complete(&mut self) {}
}

impl<In, Error> NoopObserver<In, Error> {
	pub fn new() -> Self {
		NoopObserver {
			_phantom_data: PhantomData,
		}
	}
}
