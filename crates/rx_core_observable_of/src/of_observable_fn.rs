use rx_core_traits::{Signal, SubscriptionContext};

use crate::observable::OfObservable;

/// Observable creator for [OfObservable]
pub fn of<Out, Context>(value: Out) -> OfObservable<Out, Context>
where
	Out: Signal + Clone,
	Context: SubscriptionContext,
{
	OfObservable::new(value)
}
