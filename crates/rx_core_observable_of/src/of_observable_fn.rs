use rx_core_traits::SubscriptionContext;

use crate::observable::OfObservable;

/// Observable creator for [OfObservable]
pub fn of<T, Context>(value: T) -> OfObservable<T, Context>
where
	T: Clone,
	Context: SubscriptionContext,
{
	OfObservable::new(value)
}
