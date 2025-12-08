use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle, Signal, Subscriber};

use crate::FallbackWhenSilentSubscriber;

/// The [FallbackWhenSilentOperator] calls `into()` to map incoming values to the expected
/// out value provided `From` is implemented on the downstream type.
/// When both `In` and `Out`, and `InError` and `OutError` types are the same,
/// it's equivalent to the `identity` operator and is a noop.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
	S: Scheduler,
{
	fallback: Fallback,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, Fallback, S> FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
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

impl<In, InError, Fallback, S> Operator for FallbackWhenSilentOperator<In, InError, Fallback, S>
where
	In: Signal,
	InError: Signal,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
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
