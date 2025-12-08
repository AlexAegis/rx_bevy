use rx_core_traits::Observable;

use crate::observable::ZipObservable;

pub fn zip<O1, O2>(observable_1: O1, observable_2: O2) -> ZipObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O2: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2::Out: Clone,
{
	ZipObservable::new(observable_1, observable_2)
}
