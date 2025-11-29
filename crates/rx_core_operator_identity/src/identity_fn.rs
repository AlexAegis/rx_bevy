use rx_core_traits::{Signal, SubscriptionContext};

use crate::operator::IdentityOperator;

/// It creates an IdentityOperator to easily define the input types of a
/// composite operator.
pub fn compose_operator<In, InError, Context>() -> IdentityOperator<In, InError, Context>
where
	In: Signal,
	InError: Signal,
	Context: SubscriptionContext,
{
	IdentityOperator::default()
}
