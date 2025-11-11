use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::IdentityOperator;

/// Operator creator function
pub fn identity<In, InError, Context>() -> IdentityOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	IdentityOperator::default()
}
