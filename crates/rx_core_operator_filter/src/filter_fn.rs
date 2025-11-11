use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::FilterOperator;

/// Operator creator function
pub fn filter<In, InError, Filter, Context>(
	filter: Filter,
) -> FilterOperator<In, InError, Filter, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Filter: 'static + for<'a> Fn(&'a In) -> bool + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	FilterOperator::new(filter)
}
