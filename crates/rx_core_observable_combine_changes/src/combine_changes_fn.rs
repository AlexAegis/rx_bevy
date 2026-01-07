use rx_core_common::Observable;

use crate::observable::CombineChangesObservable;

pub fn combine_changes<O1, O2>(o1: O1, o2: O2) -> CombineChangesObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	CombineChangesObservable::new(o1, o2)
}
