use rx_core_common::{ComposableOperator, Scheduler, SchedulerHandle, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::{ThrottleTimeOptions, operator::ThrottleTimeOperator};

pub trait OperatorComposeExtensionThrottleTime<T, S>: ComposableOperator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
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
	#[inline]
	fn throttle_time(
		self,
		options: ThrottleTimeOptions,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, ThrottleTimeOperator<T, Self::OutError, S>> {
		self.compose_with(ThrottleTimeOperator::new_with_options(options, scheduler))
	}
}

impl<Op, T, S> OperatorComposeExtensionThrottleTime<T, S> for Op
where
	Op: ComposableOperator<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
