use std::marker::PhantomData;

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct NoopObserver<In, Error> {
	_phantom_data: PhantomData<(In, Error)>,
}

impl<In, Error> Observer for NoopObserver<In, Error> {
	type In = In;
	type Error = Error;

	fn next(&mut self, _next: In) {}

	fn error(&mut self, _error: Self::Error) {}

	fn complete(&mut self) {}
}

impl<In, Error> NoopObserver<In, Error> {
	pub fn new() -> Self {
		NoopObserver {
			_phantom_data: PhantomData,
		}
	}
}
