use rx_core_traits::Observable;

use crate::observable::ZipObservable;

pub fn zip<O1, O2>(o1: O1, o2: O2) -> ZipObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	ZipObservable::new(o1, o2)
}
