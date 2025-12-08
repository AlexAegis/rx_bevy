use rx_core_traits::Signal;

use crate::observable::IteratorObservable;

pub trait IntoIteratorObservableExtension: IntoIterator + Clone {
	fn into_observable(self) -> IteratorObservable<Self>
	where
		Self::Item: Signal,
	{
		IteratorObservable::new(self)
	}
}

impl<T> IntoIteratorObservableExtension for T where T: IntoIterator + Clone {}
