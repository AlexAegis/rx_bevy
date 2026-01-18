use std::time::Duration;

use rx_core_common::{ComposableOperator, Scheduler, SchedulerHandle, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::DelayOperator;

pub trait OperatorComposeExtensionDelay<T, S>: ComposableOperator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [DelayOperator]
	///
	/// The `delay` operator shifts upstream values forward in time by a specified
	/// duration.
	///
	/// Upstream completion and cancellation can happen instantly if there are no
	/// pending delayed values, otherwise it will complete or cancel once all
	/// delayed values have been emitted.
	///
	/// Upstream errors are immediately propagated downstream, cancelling any
	/// pending delayed values.
	#[inline]
	fn delay(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, DelayOperator<T, Self::OutError, S>> {
		self.compose_with(DelayOperator::new(duration, scheduler))
	}
}

impl<Op, T, S> OperatorComposeExtensionDelay<T, S> for Op
where
	Op: ComposableOperator<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
