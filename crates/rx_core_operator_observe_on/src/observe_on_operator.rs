use core::marker::PhantomData;

use rx_core_common::{
	ComposableOperator, PhantomInvariant, Scheduler, SchedulerHandle, Signal, Subscriber,
};
use rx_core_macro_operator_derive::RxOperator;

use crate::ObserveOnSubscriber;

/// # [ObserveOnOperator]
///
/// The `observe_on` operator re-emits upstream `next` signals on the provided
/// scheduler.
///
/// Upstream completion and cancellation happen immediately when there are no
/// pending scheduled values, otherwise they are deferred until scheduled work
/// drains.
///
/// Upstream errors are forwarded immediately; any pending scheduled values are
/// skipped because downstream closes.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct ObserveOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, S> ObserveOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	pub fn new(scheduler: SchedulerHandle<S>) -> Self {
		Self {
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, S> ComposableOperator for ObserveOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscriber<Destination>
		= ObserveOnSubscriber<Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		ObserveOnSubscriber::new(destination, self.scheduler.clone())
	}
}
