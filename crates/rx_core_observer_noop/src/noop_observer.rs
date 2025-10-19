use std::marker::PhantomData;

use rx_core_traits::{
	Observer, ObserverInput, SignalBound, SubscriptionContext, SubscriptionLike, Teardown,
	Tickable, WithSubscriptionContext,
};

#[derive(Debug)]
pub struct NoopObserver<In, InError, Context> {
	closed: bool,
	_phantom_data: PhantomData<(In, InError, Context)>,
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

impl<In, InError, Context> Tickable for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		_tick: rx_core_traits::Tick,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
	}
}

impl<In, InError, Context> WithSubscriptionContext for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for NoopObserver<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.closed
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.closed = true;
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		teardown.execute(context);
	}
}

impl<In, InError, Context> Default for NoopObserver<In, InError, Context> {
	fn default() -> Self {
		Self {
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
