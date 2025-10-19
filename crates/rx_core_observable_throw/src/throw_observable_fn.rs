use rx_core_traits::SignalBound;

use crate::observable::ThrowObservable;

/// Observable creator for [ThrowObservable]
pub fn throw<Error, Context>(error: Error) -> ThrowObservable<Error, Context>
where
	Error: SignalBound + Clone,
{
	ThrowObservable::new(error)
}
