use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::ErrorBoundaryOperator;

/// Operator creator function
pub fn error_boundary<In, Context>() -> ErrorBoundaryOperator<In, Context>
where
	In: SignalBound,
	Context: SubscriptionContext,
{
	ErrorBoundaryOperator::default()
}
