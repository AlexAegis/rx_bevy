use rx_bevy_core::SignalContext;

use crate::IteratorObservable;

pub trait IntoIteratorObservableExtension: IntoIterator + Clone {
	fn into_observable<Context>(self) -> IteratorObservable<Self, Context>
	where
		Context: SignalContext,
	{
		IteratorObservable::new(self)
	}
}

impl<T> IntoIteratorObservableExtension for T where T: IntoIterator + Clone {}
