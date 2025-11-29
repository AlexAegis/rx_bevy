use rx_core_traits::{Observable, Signal};

use crate::observable::MergeObservable;

pub fn merge<Out, OutError, O1, O2>(
	observable_1: O1,
	observable_2: O2,
) -> MergeObservable<Out, OutError, O1, O2>
where
	Out: Signal,
	OutError: Signal,
	O1: Observable,
	O1::Out: Into<Out>,
	O1::OutError: Into<OutError>,
	O2: Observable<Context = O1::Context>,
	O2::Out: Into<Out>,
	O2::OutError: Into<OutError>,
{
	MergeObservable::new(observable_1, observable_2)
}
