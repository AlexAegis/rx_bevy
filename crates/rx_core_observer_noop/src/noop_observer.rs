use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, Signal};

#[derive_where(Default, Debug)]
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct NoopObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Observer for NoopObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn next(&mut self, _next: Self::In) {}

	fn error(&mut self, _error: Self::InError) {
		#[cfg(feature = "panic_on_error")]
		{
			panic!("noop observer observed an error!")
		}
	}

	fn complete(&mut self) {}
}
