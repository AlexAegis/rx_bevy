use rx_bevy_observable::Observable;

use crate::FlatPipe;

pub trait ObservableExtensionFlatPipe<InnerObservable, Out, Error>:
	Observable<Out = InnerObservable> + Sized
where
	InnerObservable: Observable<Out = Out, Error = Error>,
{
	fn flat(self) -> FlatPipe<Self, InnerObservable>
	where
		Self: Sized,
	{
		FlatPipe::new(self)
	}
}

impl<T, InnerObservable, Out, Error> ObservableExtensionFlatPipe<InnerObservable, Out, Error> for T
where
	T: Observable<Out = InnerObservable>,
	InnerObservable: Observable<Out = Out, Error = Error>,
{
}
