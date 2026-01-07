use rx_core_common::ComposableOperator;
use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};

use crate::operator::FindIndexOperator;

pub trait OperatorComposeExtensionFindIndex: ComposableOperator + Sized {
	#[inline]
	fn find_index<Predicate>(
		self,
		predicate: Predicate,
	) -> CompositeOperator<Self, FindIndexOperator<Self::Out, Self::OutError, Predicate>>
	where
		Predicate: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync,
	{
		self.compose_with(FindIndexOperator::new(predicate))
	}
}

impl<Op> OperatorComposeExtensionFindIndex for Op where Op: ComposableOperator {}
