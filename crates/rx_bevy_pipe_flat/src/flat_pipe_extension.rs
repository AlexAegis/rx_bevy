use rx_bevy_observable::Observable;
use rx_bevy_observable_flat::FlatForwarder;

use crate::FlatPipe;

pub trait ObservableExtensionFlatPipe<Flattener>: Observable + Sized
where
	Self: Observable<Out = Flattener::InObservable, Error = Flattener::InError>,
	Flattener: FlatForwarder,
{
	fn flat(self, flattener: Flattener) -> FlatPipe<Self, Flattener>
	where
		Self: Sized,
	{
		FlatPipe::new(self, flattener)
	}
}

impl<T, Flattener> ObservableExtensionFlatPipe<Flattener> for T
where
	Self: Observable<Out = Flattener::InObservable, Error = Flattener::InError>,
	Flattener: FlatForwarder,
{
}
