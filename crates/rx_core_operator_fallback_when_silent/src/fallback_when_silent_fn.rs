use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::FallbackWhenSilentOperator;

/// Operator creator function
pub fn fallback_when_silent<In, InError, Fallback, Context>(
	fallback: Fallback,
) -> FallbackWhenSilentOperator<In, InError, Fallback, Context>
where
	In: SignalBound,
	InError: SignalBound,
	Fallback: 'static + Fn() -> In + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	FallbackWhenSilentOperator::new(fallback)
}
