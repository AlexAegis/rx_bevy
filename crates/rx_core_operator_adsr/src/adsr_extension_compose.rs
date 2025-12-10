use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle};

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait OperatorComposeExtensionAdsr: Operator<Out = AdsrTrigger> + Sized {
	fn adsr<S>(
		self,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, AdsrOperator<Self::OutError, S>>
	where
		S: 'static + Scheduler,
	{
		CompositeOperator::new(self, AdsrOperator::new(options, scheduler))
	}
}

impl<Op> OperatorComposeExtensionAdsr for Op where Op: Operator<Out = AdsrTrigger> {}
