use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::LiftOptionOperator;

/// Operator creator function
pub fn lift_option<In, InError, Context>() -> LiftOptionOperator<In, InError, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	LiftOptionOperator::default()
}
