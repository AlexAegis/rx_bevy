use rx_core_traits::{Observable, Operator, Scheduler, SchedulerHandle};

use crate::operator::FallbackWhenSilentOperator;

pub trait ObservablePipeExtensionInto: Observable + Sized {
	#[inline]
	fn fallback_when_silent<
		Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync,
		S: 'static + Scheduler + Send + Sync,
	>(
		self,
		fallback: Fallback,
		scheduler: SchedulerHandle<S>,
	) -> <FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, S> as Operator>::OutObservable<Self>{
		FallbackWhenSilentOperator::new(fallback, scheduler).operate(self)
	}
}

impl<O> ObservablePipeExtensionInto for O where O: Observable {}
