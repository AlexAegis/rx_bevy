use rx_bevy_observable::{ForwardFlattener, Observable};

use crate::FlatPipe;

/// Provides a convenient function to flatten an observable of observables
pub trait ObservableExtensionFlatPipe: Observable + Sized {
	fn flat<Flattener>(self, flattener: Flattener) -> FlatPipe<Self, Flattener>
	where
		Self: Sized + Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
		Flattener: ForwardFlattener,
	{
		FlatPipe::new(self, flattener)
	}
}

impl<T> ObservableExtensionFlatPipe for T where T: Observable {}
