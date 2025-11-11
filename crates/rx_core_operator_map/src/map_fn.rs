use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::MapOperator;

/// Operator creator function
pub fn map<In, InError, Mapper, Out, Context>(
	mapper: Mapper,
) -> MapOperator<In, InError, Mapper, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Out: SignalBound,
	Mapper: Clone + Fn(In) -> Out + Send + Sync,
	Context: SubscriptionContext,
{
	MapOperator::new(mapper)
}
