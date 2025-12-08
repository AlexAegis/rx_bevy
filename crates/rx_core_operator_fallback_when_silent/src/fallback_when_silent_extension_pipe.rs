use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Scheduler, SchedulerHandle};

use crate::operator::FallbackWhenSilentOperator;

pub trait ObservablePipeExtensionInto: Observable + Sized {
	fn fallback_when_silent<
		Fallback: 'static + Fn() -> Self::Out + Clone + Send + Sync,
		S: 'static + Scheduler + Send + Sync,
	>(
		self,
		fallback: Fallback,
		scheduler: SchedulerHandle<S>,
	) -> Pipe<Self, FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, S>> {
		Pipe::new(self, FallbackWhenSilentOperator::new(fallback, scheduler))
	}
}

impl<O> ObservablePipeExtensionInto for O where O: Observable {}
