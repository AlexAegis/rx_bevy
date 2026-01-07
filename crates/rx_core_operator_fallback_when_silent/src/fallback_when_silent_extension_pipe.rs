use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, WorkContextProvider};

use crate::operator::FallbackWhenSilentOperator;

pub trait ObservablePipeExtensionInto<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn fallback_when_silent<
		Fallback: 'static
			+ Fn(S::Tick, &mut <S::WorkContextProvider as WorkContextProvider>::Item<'_>, usize) -> Self::Out
			+ Clone
			+ Send
			+ Sync,
		S: 'static + Scheduler + Send + Sync,
	>(
		self,
		fallback: Fallback,
		scheduler: SchedulerHandle<S>,
	) -> <FallbackWhenSilentOperator<Self::Out, Self::OutError, Fallback, S> as Operator<'o>>::OutObservable<Self>{
		FallbackWhenSilentOperator::new(fallback, scheduler).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionInto<'o> for O where O: 'o + Observable + Send + Sync {}
