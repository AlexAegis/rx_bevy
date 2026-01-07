use rx_core_common::Observable;

use crate::observable::DeferredObservable;

pub fn deferred_observable<F, Source>(observable_creator: F) -> DeferredObservable<F, Source>
where
	Source: Observable,
	F: Clone + FnMut() -> Source,
{
	DeferredObservable::new(observable_creator)
}
