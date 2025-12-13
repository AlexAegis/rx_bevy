use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FindIndexOperator;

pub trait OperatorComposeExtensionFindIndex: Operator + Sized {
	fn find_index<P>(
		self,
		predicate: P,
	) -> CompositeOperator<Self, FindIndexOperator<Self::Out, Self::OutError, P>>
	where
		P: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		CompositeOperator::new(self, FindIndexOperator::new(predicate))
	}
}

impl<Op> OperatorComposeExtensionFindIndex for Op where Op: Operator {}
