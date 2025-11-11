use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::ScanOperator;

/// Operator creator function
pub fn scan<In, InError, Reducer, Out, Context>(
	reducer: Reducer,
	seed: Out,
) -> ScanOperator<In, InError, Reducer, Out, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out + Send + Sync + Clone,
	Out: SignalBound + Clone,
	Context: SubscriptionContext,
{
	ScanOperator::new(reducer, seed)
}
