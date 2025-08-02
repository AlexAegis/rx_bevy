use std::time::Duration;

#[cfg(feature = "bevy")]
use bevy_ecs::{event::Event, system::Res};
#[cfg(feature = "bevy")]
use bevy_time::Time;
#[cfg(feature = "bevy")]
use rx_bevy_common_bounds::Clock;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Used for scheduling, the subscriptions are ticked with this event
/// ? Could be generic over Schedule or something thats associated with the observer
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Event))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Tick {
	pub now: Duration,
	pub delta: Duration,
}

impl Tick {
	// Move this generic to Tick itself, and make the tick function generic
	#[cfg(feature = "bevy")]
	pub fn new<C: Clock>(time: &Res<Time<C>>) -> Self {
		Self {
			now: time.elapsed(),
			delta: time.delta(),
		}
	}
}
