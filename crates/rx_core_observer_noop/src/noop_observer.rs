use derive_where::derive_where;
use rx_core_common::{PhantomInvariant, RxObserver, Signal};
use rx_core_macro_observer_derive::RxObserver;

/// # NoopObserver
///
/// Does nothing except panics in dev mode when an error is observed.
#[derive_where(Default, Debug)]
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
pub struct NoopObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> RxObserver for NoopObserver<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn next(&mut self, _next: Self::In) {}

	#[inline]
	fn error(&mut self, _error: Self::InError) {
		debug_assert!(false, "noop observer observed an uncaught error!")
	}

	#[inline]
	fn complete(&mut self) {}
}
