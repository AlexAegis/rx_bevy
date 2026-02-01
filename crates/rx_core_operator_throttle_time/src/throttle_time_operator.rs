use core::marker::PhantomData;
use std::time::Duration;

use rx_core_common::{
	ComposableOperator, PhantomInvariant, Scheduler, SchedulerHandle, Signal, Subscriber,
};
use rx_core_macro_operator_derive::RxOperator;

use crate::{ThrottleTimeOptions, ThrottleTimeSubscriber};

/// # [ThrottleTimeOperator]
///
/// The `throttle_time` operator limits the frequency of downstream
/// emissions by emitting an upstream value, then suppressing subsequent
/// emissions until the duration elapses.
///
/// When the output is set to `LeadingOnly`, the first upstream value in a
/// throttle window is emitted immediately. When the output is set to
/// `TrailingOnly`, the most recent upstream value observed during the throttle
/// window is emitted when it ends. The default `LeadingAndTrailing` setting
/// emits both the first and the most recent values in each throttle window.
///
/// Upstream completion and cancellation can happen instantly if there is no
/// pending trailing value, otherwise it will complete or cancel once the
/// trailing value has been emitted.
///
/// Upstream errors are immediately propagated downstream, cancelling any
/// pending throttled value.
///
/// ## Options
///
/// Use [ThrottleTimeOptions] to configure `duration` and output behavior.
///
/// - `duration`: The throttle window duration.
///   Default: `1s`.
/// - `output`: Controls which emissions are produced in each throttle window.
///   Default: `ThrottleOutputBehavior::LeadingAndTrailing`. Possible values:
///   `ThrottleOutputBehavior::LeadingOnly`,
///   `ThrottleOutputBehavior::TrailingOnly`,
///   `ThrottleOutputBehavior::LeadingAndTrailing`.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct ThrottleTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	options: ThrottleTimeOptions,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, S> ThrottleTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	pub fn new(duration: Duration, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			options: ThrottleTimeOptions::new(duration),
			scheduler,
			_phantom_data: PhantomData,
		}
	}

	pub fn new_with_options(options: ThrottleTimeOptions, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			options,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, S> ComposableOperator for ThrottleTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscriber<Destination>
		= ThrottleTimeSubscriber<Destination, S>
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
		ThrottleTimeSubscriber::new(destination, self.options, self.scheduler.clone())
	}
}
