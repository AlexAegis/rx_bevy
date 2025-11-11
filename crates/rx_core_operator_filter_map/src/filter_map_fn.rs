use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::FilterMapOperator;

/// Operator creator function
pub fn filter_map<In, InError, Mapper, Out, Context>(
	mapper: Mapper,
) -> FilterMapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Mapper: 'static + Fn(In) -> Option<Out> + Clone + Send + Sync,
	Out: SignalBound,
	Context: SubscriptionContext,
{
	FilterMapOperator::new(mapper)
}
