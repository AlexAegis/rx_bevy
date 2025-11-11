use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::TakeOperator;

/// Operator creator function
pub fn take<In, InError, Context>(count: usize) -> TakeOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	TakeOperator::new(count)
}
