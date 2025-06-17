use crate::IteratorObservable;

pub trait IntoIteratorObservableExtension: IntoIterator + Clone {
	fn into_observable(self) -> IteratorObservable<Self, Self::Item> {
		IteratorObservable::new(self)
	}
}

impl<T> IntoIteratorObservableExtension for T where T: IntoIterator + Clone {}
