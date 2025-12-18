use rx_core_traits::{Observable, Operator, Scheduler, SchedulerHandle};

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait ObservablePipeExtensionAdsr: Observable<Out = AdsrTrigger> + Sized {
	#[inline]
	fn adsr<S>(
		self,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> <AdsrOperator<Self::OutError, S> as Operator>::OutObservable<Self>
	where
		S: Scheduler,
	{
		AdsrOperator::new(options, scheduler).operate(self)
	}
}

impl<O> ObservablePipeExtensionAdsr for O where O: Observable<Out = AdsrTrigger> {}
