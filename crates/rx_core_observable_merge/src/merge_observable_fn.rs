use rx_core_observable_erased::ErasedObservables;
use rx_core_traits::Signal;

use crate::observable::MergeObservable;

pub fn merge<Out, OutError, const SIZE: usize>(
	observables: impl Into<ErasedObservables<Out, OutError, SIZE>>,
	concurrency_limit: usize,
) -> MergeObservable<Out, OutError, SIZE>
where
	Out: Signal,
	OutError: Signal,
{
	MergeObservable::new(observables, concurrency_limit)
}
