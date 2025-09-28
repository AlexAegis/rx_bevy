use rx_bevy_core::DropContext;

use crate::IteratorObservable;

pub trait IntoIteratorObservableExtension: IntoIterator + Clone {
	fn into_observable<Context>(self) -> IteratorObservable<Self, Context>
	where
		Context: DropContext,
	{
		IteratorObservable::new(self)
	}
}

impl<T> IntoIteratorObservableExtension for T where T: IntoIterator + Clone {}
