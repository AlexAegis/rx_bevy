use bevy_ecs::system::Commands;
use rx_bevy_common_bounds::SignalBound;
use rx_bevy_core::Observer;

use crate::RxSubscriberEvent;

pub trait ObserverSignalPush<In, InError>
where
	Self: Observer<In = In, InError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	fn push<'a, 'w, 's>(
		&mut self,
		signal: impl Into<RxSubscriberEvent<In, InError>>,
		commands: &'a mut Commands<'w, 's>,
	) {
		match signal.into() {
			RxSubscriberEvent::Next(next) => self.next(next, ChannelContext { commands }),
			RxSubscriberEvent::Error(error) => self.error(error, ChannelContext { commands }),
			RxSubscriberEvent::Complete => self.complete(ChannelContext { commands }),
			RxSubscriberEvent::Tick(tick) => self.tick(tick, ChannelContext { commands }),
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
