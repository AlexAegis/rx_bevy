use rx_core_traits::Observable;

use crate::observable::CombineLatestObservable;

pub fn combine_latest<O1, O2>(observable_1: O1, observable_2: O2) -> CombineLatestObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	CombineLatestObservable::new(observable_1, observable_2)
}
