use core::marker::PhantomData;

use rx_core_common::{
	ComposableOperator, PhantomInvariant, Scheduler, SchedulerHandle, Signal, Subscriber,
	WorkContextProvider,
};
use rx_core_macro_operator_derive::RxOperator;

use crate::FallbackWhenSilentSubscriber;

/// The [FallbackWhenSilentOperator] calls `into()` to map incoming values to the expected
/// output value, provided `From` is implemented on the downstream type.
/// When `In` and `Out`, as well as `InError` and `OutError`, are the same types,
/// it is equivalent to the `identity` operator and is a no-op.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static
		+ Fn(S::Tick, &mut <S::WorkContextProvider as WorkContextProvider>::Item<'_>, usize) -> In
		+ Clone
		+ Send
		+ Sync,
	S: Scheduler,
{
	fallback: Fallback,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, Fallback, S> FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static
		+ Fn(S::Tick, &mut <S::WorkContextProvider as WorkContextProvider>::Item<'_>, usize) -> In
		+ Clone
		+ Send
		+ Sync,
	S: Scheduler,
{
	pub fn new(fallback: Fallback, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			fallback,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Fallback, S> ComposableOperator
	for FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static
		+ Fn(S::Tick, &mut <S::WorkContextProvider as WorkContextProvider>::Item<'_>, usize) -> In
		+ Clone
		+ Send
		+ Sync,
	S: 'static + Scheduler + Send,
{
	type Subscriber<Destination>
		= FallbackWhenSilentSubscriber<In, InError, Fallback, Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		FallbackWhenSilentSubscriber::new(
			destination,
			self.fallback.clone(),
			self.scheduler.clone(),
		)
	}
}
