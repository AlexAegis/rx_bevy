use rx_core_traits::Observable;

use crate::observable::JoinObservable;

/// # [JoinObservable]
///
/// This observable will only emit once both of it's input observables
/// have completed. After which it will emit a tuple of the last emissions
/// from each input observable, then complete.
///
/// Meaning if even one of the observables haven't emitted before all of them
/// had completed, only a complete notification will be observed!
///
/// If not all observables complete, nothing will be emitted even if all
/// input observables were primed.
pub fn join<O1, O2>(o1: O1, o2: O2) -> JoinObservable<O1, O2>
where
	O1: 'static + Send + Sync + Observable,
	O1::Out: Clone,
	O2: 'static + Send + Sync + Observable,
	O2::Out: Clone,
	O2::OutError: Into<O1::OutError>,
{
	JoinObservable::new(o1, o2)
}
