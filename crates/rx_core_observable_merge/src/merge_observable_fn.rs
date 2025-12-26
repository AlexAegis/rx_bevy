use rx_core_observable_erased::ErasedObservables;
use rx_core_traits::Signal;

use crate::observable::MergeObservable;

/// # MergeObservable
///
/// > Category: Combination Observable
///
/// Subscribes to multiple input observables of shared output types, and
/// joins their outputs into a single stream.
///
/// ## Completion Behavior
///
/// Completes once when all input observables completed.
///
/// ## Error Behavior
///
/// Errors when any of the input observables errored.
///
/// ## Arguments
///
/// - `observables`: A tuple or array of observables of shared output types
/// - `concurrency_limit`: How many observables can be subscribed to at a time.
///   Subscriptions will be established in the same order as the observables
///   were supplied.
///   - If you want all your input observables to be subscribed immediately,
///     make sure this `concurrency_limit` is set to an equal or greater number
///     than the number of observables defined in the `observables` argument,
///     for example by setting it to `usize::MAX`.
///   - If the `concurrency_limit` is lower than the number of observables
///     defined in the `observables` argument, for example with a
///     `concurrency_limit` of 2 and an `observables` list of 3, only the first
///     two observables will be subscribed to. The third one will only be
///     subscribed to when one of the first 2 completes.
///   - Setting `concurrency_limit` to just 1 is functionally equivalent of
///     just using [ConcatObservable](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_concat).
///   - Setting `concurrency_limit` to 0 is invalid, and 1 will be used instead
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
