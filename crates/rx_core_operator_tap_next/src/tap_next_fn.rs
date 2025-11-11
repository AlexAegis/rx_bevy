use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::TapNextOperator;

/// Operator creator function
pub fn tap_next<In, InError, OnNext, Context>(
	callback: OnNext,
) -> TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_, '_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	TapNextOperator::new(callback)
}
