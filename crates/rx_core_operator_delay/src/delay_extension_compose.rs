use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Scheduler, Signal, WithSubscriptionContext};

use crate::operator::{DelayOperator, DelayOperatorOptions};

pub trait OperatorComposeExtensionDelay<T, S>: Operator<Out = T> + Sized
where
	T: Signal,
	S: 'static
		+ Scheduler<ContextProvider = <Self as WithSubscriptionContext>::Context>
		+ Send
		+ Sync,
{
	fn delay(
		self,
		options: DelayOperatorOptions<S>,
	) -> CompositeOperator<Self, DelayOperator<T, Self::OutError, Self::Context, S>> {
		CompositeOperator::new(self, DelayOperator::new(options))
	}
}

impl<Op, T, S> OperatorComposeExtensionDelay<T, S> for Op
where
	Op: Operator<Out = T>,
	T: Signal,
	S: 'static
		+ Scheduler<ContextProvider = <Self as WithSubscriptionContext>::Context>
		+ Send
		+ Sync,
{
}
