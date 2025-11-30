use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::FilterMapOperator;

pub trait OperatorComposeExtensionFilterMap: Operator + Sized {
	fn filter_map<
		NextOut: Signal,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> CompositeOperator<
		Self,
		FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>,
	> {
		CompositeOperator::new(self, FilterMapOperator::new(mapper))
	}
}

impl<Op> OperatorComposeExtensionFilterMap for Op where Op: Operator {}
