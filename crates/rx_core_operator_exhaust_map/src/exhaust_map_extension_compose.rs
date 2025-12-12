use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ExhaustMapOperator;

pub trait OperatorComposeExtensionExhaustMap: Operator + Sized {
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		ExhaustMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>,
	>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		CompositeOperator::new(self, ExhaustMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionExhaustMap for Op where Op: Operator {}
