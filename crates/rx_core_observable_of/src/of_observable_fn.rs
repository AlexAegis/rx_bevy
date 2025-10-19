use crate::observable::OfObservable;

/// Observable creator for [OfObservable]
pub fn of<T>(value: T) -> OfObservable<T, ()>
where
	T: Clone,
{
	OfObservable::new(value)
}
