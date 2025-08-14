use rx_bevy_observable::Observer;

use crate::{RxSubscriberEvent, SignalBound};

pub trait ObserverSignalPush<In, InError>
where
	Self: Observer<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	fn push(&mut self, signal: impl Into<RxSubscriberEvent<In, InError>>) {
		match signal.into() {
			RxSubscriberEvent::Next(next) => self.next(next),
			RxSubscriberEvent::Error(error) => self.error(error),
			RxSubscriberEvent::Complete => self.complete(),
			_ => {}
		}
	}
}

impl<T, In, InError> ObserverSignalPush<In, InError> for T
where
	T: Observer<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
}
