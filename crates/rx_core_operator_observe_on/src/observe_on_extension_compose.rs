use rx_core_common::{ComposableOperator, Scheduler, SchedulerHandle, Signal};
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::ObserveOnOperator;

pub trait OperatorComposeExtensionObserveOn<T, S>: ComposableOperator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
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
	#[inline]
	fn observe_on(
		self,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, ObserveOnOperator<T, Self::OutError, S>> {
		self.compose_with(ObserveOnOperator::new(scheduler))
	}
}

impl<Op, T, S> OperatorComposeExtensionObserveOn<T, S> for Op
where
	Op: ComposableOperator<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
