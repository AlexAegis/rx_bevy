use std::marker::PhantomData;

use rx_bevy_observable::{Observer, ObserverInput};

#[derive(Debug)]
pub struct NoopObserver<In, InError> {
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ObserverInput for NoopObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError> Observer for NoopObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	fn next(&mut self, _next: Self::In) {}

	fn error(&mut self, _error: Self::InError) {}

	fn complete(&mut self) {}
}

impl<In, InError> Default for NoopObserver<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
