use rx_bevy_observable::Observable;
use rx_bevy_observable_flat::ForwardFlattener;

use crate::FlatPipe;

pub trait ObservableExtensionFlatPipe<Flattener>: Observable + Sized
where
	Self: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener,
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
	Self: Observable<Out = Flattener::InObservable, OutError = Flattener::InError>,
	Flattener: ForwardFlattener,
{
}
