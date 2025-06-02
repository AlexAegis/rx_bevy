use std::marker::PhantomData;

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct NoopObserver<In> {
	_phantom_data: PhantomData<In>,
}

impl<In> Observer for NoopObserver<In> {
	type In = In;
	type Error = ();

	fn next(&mut self, _next: In) {}

	fn error(&mut self, _error: Self::Error) {}

	fn complete(&mut self) {}
}

impl<In> NoopObserver<In> {
	pub fn new() -> Self {
		NoopObserver {
			_phantom_data: PhantomData,
		}
	}
}
