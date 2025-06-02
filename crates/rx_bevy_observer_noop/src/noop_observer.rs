use std::marker::PhantomData;

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct NoopObserver<In> {
	_phantom_data: PhantomData<In>,
}

impl<In> Observer for NoopObserver<In> {
	type In = In;
	type Error = ();

	fn on_push(&mut self, _value: In) {}

	fn on_error(&mut self, _error: Self::Error) {}

	fn on_complete(&mut self) {}
}

impl<In> NoopObserver<In> {
	pub fn new() -> Self {
		NoopObserver {
			_phantom_data: PhantomData,
		}
	}
}
