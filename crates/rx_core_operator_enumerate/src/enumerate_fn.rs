use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::EnumerateOperator;

/// Operator creator function
pub fn enumerate<In, InError, Context>() -> EnumerateOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	EnumerateOperator::default()
}
