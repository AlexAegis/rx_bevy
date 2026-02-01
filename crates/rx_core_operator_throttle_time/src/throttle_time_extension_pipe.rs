use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::{ThrottleTimeOptions, operator::ThrottleTimeOperator};

pub trait ObservablePipeExtensionThrottleTime<'o, T, S>:
	'o + Observable<Out = T> + Sized + Send + Sync
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
	) -> <ThrottleTimeOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		ThrottleTimeOperator::new_with_options(options, scheduler).operate(self)
	}
}

impl<'o, O, T, S> ObservablePipeExtensionThrottleTime<'o, T, S> for O
where
	O: 'o + Observable<Out = T> + Send + Sync,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
