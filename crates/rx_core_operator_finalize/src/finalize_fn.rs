use rx_core_traits::{SignalBound, SubscriptionContext};

use crate::operator::FinalizeOperator;

/// Operator creator function
pub fn finalize<Out, OutError, Callback, Context>(
	callback: Callback,
) -> FinalizeOperator<Out, OutError, Callback, Context>
where
	Out: SignalBound,
	OutError: SignalBound,
	Callback: 'static + Clone + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	FinalizeOperator::new(callback)
}
