use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::MapOperator;

pub trait OperatorComposeExtensionMap: Operator + Sized {
	fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut>> {
		CompositeOperator::new(self, MapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionMap for Op where Op: Operator {}
