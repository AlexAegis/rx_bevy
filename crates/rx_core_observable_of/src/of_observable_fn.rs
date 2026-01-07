use rx_core_common::Signal;

use crate::observable::OfObservable;

pub fn of<Out>(value: Out) -> OfObservable<Out>
where
	Out: Signal + Clone,
{
	OfObservable::new(value)
}
