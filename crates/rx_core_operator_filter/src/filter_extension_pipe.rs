use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SubscriptionContext};

use crate::operator::FilterOperator;

/// Operator creator function
pub fn filter<Error, Filter, Out, Context>(
	filter: Filter,
) -> FilterOperator<Out, Error, Filter, Context>
where
	Filter: Fn(&Out) -> bool + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	FilterOperator::new(filter)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFilter: Observable + Sized {
	fn filter<Filter: 'static + Fn(&Self::Out) -> bool + Clone + Send + Sync>(
		self,
		filter: Filter,
	) -> Pipe<Self, FilterOperator<Self::Out, Self::OutError, Filter, Self::Context>> {
		Pipe::new(self, FilterOperator::new(filter))
	}
}

impl<T> ObservableExtensionFilter for T where T: Observable {}
