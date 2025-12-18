use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::FindOperator;

pub trait OperatorComposeExtensionFind: ComposableOperator + Sized {
	#[inline]
	fn find<Predicate>(
		self,
		predicate: Predicate,
	) -> CompositeOperator<Self, FindOperator<Self::Out, Self::OutError, Predicate>>
	where
		Predicate: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		self.compose_with(FindOperator::new(predicate))
	}
}

impl<Op> OperatorComposeExtensionFind for Op where Op: ComposableOperator {}
