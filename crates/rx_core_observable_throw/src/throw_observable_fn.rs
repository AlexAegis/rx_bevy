use rx_core_traits::{Signal, SubscriptionContext};

use crate::observable::ThrowObservable;

/// Observable creator for [ThrowObservable]
pub fn throw<Error, Context>(error: Error) -> ThrowObservable<Error, Context>
where
	Error: Signal + Clone,
	Context: SubscriptionContext,
{
	ThrowObservable::new(error)
}
