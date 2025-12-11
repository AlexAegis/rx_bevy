use std::time::Duration;

use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::DelayOperator;

pub trait OperatorComposeExtensionDelay<T, S>: Operator<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	fn delay(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, DelayOperator<T, Self::OutError, S>> {
		CompositeOperator::new(self, DelayOperator::new(duration, scheduler))
	}
}

impl<Op, T, S> OperatorComposeExtensionDelay<T, S> for Op
where
	Op: Operator<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
