use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::ComposableOperator;

use crate::operator::FilterOperator;

pub trait OperatorComposeExtensionFilter: ComposableOperator + Sized {
	#[inline]
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> CompositeOperator<Self, FilterOperator<Self::Out, Self::OutError, Filter>> {
		self.compose_with(FilterOperator::new(filter))
	}
}

impl<Op> OperatorComposeExtensionFilter for Op where Op: ComposableOperator {}
