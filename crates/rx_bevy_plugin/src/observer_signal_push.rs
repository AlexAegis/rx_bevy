use rx_bevy_observable::Observer;

use crate::{ObservableSignalBound, RxSignal};

pub trait ObserverSignalPush<In, InError>
where
	Self: Observer<In = In, InError = InError>,
	In: ObservableSignalBound,
	InError: ObservableSignalBound,
{
	fn push(&mut self, signal: RxSignal<In, InError>) {
		match signal {
			RxSignal::Next(next) => self.next(next),
			RxSignal::Error(error) => self.error(error),
			RxSignal::Complete => self.complete(),
		}
	}
}

impl<T, In, InError> ObserverSignalPush<In, InError> for T
where
	T: Observer<In = In, InError = InError>,
	In: ObservableSignalBound,
	InError: ObservableSignalBound,
{
}
