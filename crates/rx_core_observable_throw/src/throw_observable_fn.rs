use rx_core_common::Signal;

use crate::observable::ThrowObservable;

/// Observable creator for [ThrowObservable]
pub fn throw<Error>(error: Error) -> ThrowObservable<Error>
where
	Error: Signal + Clone,
{
	ThrowObservable::new(error)
}
