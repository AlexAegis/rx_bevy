use std::time::Duration;

use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Scheduler, SchedulerHandle, Signal};

use crate::operator::DelayOperator;

pub trait OperatorComposeExtensionDelay<T, S>: ComposableOperator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
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
