use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MergeMapOperator;

pub trait OperatorComposeExtensionMergeMap: Operator + Sized {
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Switcher: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		switcher: Switcher,
	) -> CompositeOperator<
		Self,
		MergeMapOperator<Self::Out, Self::OutError, Switcher, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, MergeMapOperator::new(switcher))
	}
}

impl<Op> OperatorComposeExtensionMergeMap for Op where Op: Operator {}
