use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FilterOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionFilter: Operator + Sized {
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> CompositeOperator<Self, FilterOperator<Self::Out, Self::OutError, Filter, Self::Context>>
	{
		CompositeOperator::new(self, FilterOperator::new(filter))
	}
}

impl<T> CompositeOperatorExtensionFilter for T where T: Operator {}
