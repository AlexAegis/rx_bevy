use derive_where::derive_where;
use rx_core_traits::Observable;

/// Completions and unsubscribes are present here because they don't have an
/// argument and there'd be no way to tell which observable had completed.
/// But through the `next` function, both information can be signaled.
/// TODO: rename to Either2, and just generic, no observables mentioned, Variant1, Variant2
#[derive_where(Debug; O1::Out, O2::Out)]
pub enum EitherOut2<O1, O2>
where
	O1: Observable,
	O2: Observable,
{
	O1(O1::Out),
	CompleteO1,
	UnsubscribeO1,
	O2(O2::Out),
	CompleteO2,
	UnsubscribeO2,
}
