use std::time::Duration;

use rx_core_common::{ComposableOperator, Scheduler, SchedulerHandle, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::DebounceTimeOperator;

pub trait OperatorComposeExtensionDebounceTime<T, S>: ComposableOperator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [DebounceTimeOperator]
	///
	/// The `debounce_time` operator emits the most recent upstream value only
	/// after the specified duration passes without another emission.
	///
	/// Upstream completion and cancellation can happen instantly if there are
	/// no pending debounced values, otherwise it will complete or cancel once
	/// the pending debounced value has been emitted.
	///
	/// Upstream errors are immediately propagated downstream, cancelling any
	/// pending debounced value.
	#[inline]
	fn debounce_time(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, DebounceTimeOperator<T, Self::OutError, S>> {
		self.compose_with(DebounceTimeOperator::new(duration, scheduler))
	}
}

impl<Op, T, S> OperatorComposeExtensionDebounceTime<T, S> for Op
where
	Op: ComposableOperator<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
