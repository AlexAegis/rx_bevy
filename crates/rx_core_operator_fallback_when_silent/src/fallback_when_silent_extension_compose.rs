use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle};

use crate::operator::FallbackWhenSilentOperator;

pub trait OperatorComposeExtensionFallbackWhenSilent: Operator + Sized {
	fn fallback_when_silent<
		Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync,
		S: 'static + Scheduler + Send + Sync,
	>(
		self,
		fallback: Fallback,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, S>>
	{
		CompositeOperator::new(self, FallbackWhenSilentOperator::new(fallback, scheduler))
	}
}

impl<Op> OperatorComposeExtensionFallbackWhenSilent for Op where Op: Operator {}
