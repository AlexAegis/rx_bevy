use rx_core_traits::Signal;

use crate::observable::OfObservable;

/// Observable creator for [OfObservable]
pub fn of<Out>(value: Out) -> OfObservable<Out>
where
	Out: Signal + Clone,
{
	OfObservable::new(value)
}
