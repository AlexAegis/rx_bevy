use std::marker::PhantomData;

use rx_bevy_core::{Observer, ObserverInput, UpgradeableObserver, prelude::ObserverSubscriber};

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

	fn error(&mut self, _error: Self::InError) {
		#[cfg(feature = "panic_on_error")]
		{
			panic!("noop observer observed an error!")
		}
	}

	fn complete(&mut self) {}

	#[cfg(feature = "tick")]
	fn tick(&mut self, _tick: rx_bevy_core::Tick) {}
}

impl<In, InError> UpgradeableObserver for NoopObserver<In, InError>
where
	In: 'static,
	InError: 'static,
{
	type Subscriber = ObserverSubscriber<Self>;

	fn upgrade(self) -> Self::Subscriber {
		ObserverSubscriber::new(self)
	}
}

impl<In, InError> Default for NoopObserver<In, InError> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
