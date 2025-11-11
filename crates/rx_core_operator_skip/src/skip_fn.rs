use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::SkipOperator;

/// Operator creator function
pub fn skip<In, InError, Context>(count: usize) -> SkipOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	SkipOperator::new(count)
}
