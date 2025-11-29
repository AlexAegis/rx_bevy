use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_observer_derive::RxObserver;
use rx_core_traits::{Observer, Signal, SubscriptionContext};

#[derive_where(Default, Debug)]
#[derive(RxObserver)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_context(Context)]
pub struct NoopObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<(In, InError, fn(Context))>,
}

impl<In, InError, Context> Observer for NoopObserver<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	fn next(
		&mut self,
		_next: Self::In,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
	}

	fn error(
		&mut self,
		_error: Self::InError,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		#[cfg(feature = "panic_on_error")]
		{
			panic!("noop observer observed an error!")
		}
	}

	fn complete(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {}
}
