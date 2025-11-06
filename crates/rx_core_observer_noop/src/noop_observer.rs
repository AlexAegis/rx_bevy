use core::marker::PhantomData;

use rx_core_traits::{
	DetachedSubscriber, Observer, ObserverInput, PrimaryCategoryObserver, SignalBound,
	SubscriptionContext, UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

#[derive(Debug)]
pub struct NoopObserver<In, InError, Context> {
	_phantom_data: PhantomData<(In, InError, fn(Context))>,
}

impl<In, InError, Context> ObserverInput for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Context> Observer for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
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

impl<In, InError, Context> WithSubscriptionContext for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> WithPrimaryCategory for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type PrimaryCategory = PrimaryCategoryObserver;
}

impl<In, InError, Context> UpgradeableObserver for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Upgraded = DetachedSubscriber<Self>;

	fn upgrade(self) -> Self::Upgraded {
		DetachedSubscriber::new(self)
	}
}

impl<In, InError, Context> Default for NoopObserver<In, InError, Context> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
