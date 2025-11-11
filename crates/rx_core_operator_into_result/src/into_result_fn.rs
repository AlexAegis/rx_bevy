use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::IntoResultOperator;

/// Operator creator function
pub fn into_result<In, InError, Context>() -> IntoResultOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	IntoResultOperator::default()
}
