use std::marker::PhantomData;

use rx_bevy_observable::Observer;

#[derive(Default, Debug)]
pub struct NoopObserver<In> {
	_phantom_data_in: PhantomData<In>,
}

impl<In> Observer<In> for NoopObserver<In> {
	fn on_push(&mut self, _value: In) {}
}

impl<In> NoopObserver<In> {
	pub fn new() -> Self {
		NoopObserver {
			_phantom_data_in: PhantomData,
		}
	}
}
