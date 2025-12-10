use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Scheduler, SchedulerHandle};

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait ObservablePipeExtensionAdsr: Observable<Out = AdsrTrigger> + Sized {
	fn adsr<S>(
		self,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> Pipe<Self, AdsrOperator<Self::OutError, S>>
	where
		S: Scheduler,
	{
		Pipe::new(self, AdsrOperator::new(options, scheduler))
	}
}

impl<O> ObservablePipeExtensionAdsr for O where O: Observable<Out = AdsrTrigger> {}
