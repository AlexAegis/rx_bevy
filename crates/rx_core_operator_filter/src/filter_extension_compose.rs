use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FilterOperator;

pub trait OperatorComposeExtensionFilter: Operator + Sized {
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> CompositeOperator<Self, FilterOperator<Self::Out, Self::OutError, Filter, Self::Context>>
	{
		CompositeOperator::new(self, FilterOperator::new(filter))
	}
}

impl<Op> OperatorComposeExtensionFilter for Op where Op: Operator {}
