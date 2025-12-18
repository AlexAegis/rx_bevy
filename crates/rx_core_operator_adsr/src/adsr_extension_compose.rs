use rx_core_operator_composite::operator::*;
use rx_core_traits::{ComposableOperator, Scheduler, SchedulerHandle};

use crate::{
	AdsrTrigger,
	operator::{AdsrOperator, AdsrOperatorOptions},
};

pub trait OperatorComposeExtensionAdsr: ComposableOperator<Out = AdsrTrigger> + Sized {
	#[inline]
	fn adsr<S>(
		self,
		options: AdsrOperatorOptions,
		scheduler: SchedulerHandle<S>,
	) -> CompositeOperator<Self, AdsrOperator<Self::OutError, S>>
	where
		S: 'static + Scheduler,
	{
		self.compose_with(AdsrOperator::new(options, scheduler))
	}
}

impl<Op> OperatorComposeExtensionAdsr for Op where Op: ComposableOperator<Out = AdsrTrigger> {}
