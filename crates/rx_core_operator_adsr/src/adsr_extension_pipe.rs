use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle};

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait ObservablePipeExtensionAdsr<'o>:
	'o + Observable<Out = AdsrTrigger> + Sized + Send + Sync
{
	#[inline]
	fn adsr<S>(
		self,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> <AdsrOperator<Self::OutError, S> as Operator<'o>>::OutObservable<Self>
	where
		S: Scheduler,
	{
		AdsrOperator::new(options, scheduler).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionAdsr<'o> for O where
	O: 'o + Observable<Out = AdsrTrigger> + Send + Sync
{
}
