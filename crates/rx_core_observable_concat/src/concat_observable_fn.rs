use rx_core_common::{ErasedObservables, Signal};

use crate::observable::ConcatObservable;

pub fn concat<Out, OutError, const SIZE: usize>(
	observables: impl Into<ErasedObservables<Out, OutError, SIZE>>,
) -> ConcatObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	ConcatObservable::new(observables)
}
