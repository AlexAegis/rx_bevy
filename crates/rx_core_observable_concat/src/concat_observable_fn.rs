use rx_core_observable_erased::ErasedObservables;
use rx_core_traits::Signal;

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
