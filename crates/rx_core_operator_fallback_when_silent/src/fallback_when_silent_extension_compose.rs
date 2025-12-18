use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Scheduler, SchedulerHandle};

use crate::operator::FallbackWhenSilentOperator;

pub trait OperatorComposeExtensionFallbackWhenSilent: ComposableOperator + Sized {
	#[inline]
	fn fallback_when_silent<
		Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync,
		S: 'static + Scheduler + Send + Sync,
	>(
		self,
		fallback: Fallback,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, S>>
	{
		self.compose_with(FallbackWhenSilentOperator::new(fallback, scheduler))
	}
}

impl<Op> OperatorComposeExtensionFallbackWhenSilent for Op where Op: ComposableOperator {}
