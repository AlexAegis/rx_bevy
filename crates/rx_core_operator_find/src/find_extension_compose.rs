use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FindOperator;

pub trait OperatorComposeExtensionFind: Operator + Sized {
	fn find<P>(
		self,
		predicate: P,
	) -> CompositeOperator<Self, FindOperator<Self::Out, Self::OutError, P>>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		CompositeOperator::new(self, FindOperator::new(predicate))
	}
}

impl<Op> OperatorComposeExtensionFind for Op where Op: Operator {}
