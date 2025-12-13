use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ConcatMapOperator;

pub trait OperatorComposeExtensionConcatMap: Operator + Sized {
	fn switch_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		ConcatMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, ConcatMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionConcatMap for Op where Op: Operator {}
