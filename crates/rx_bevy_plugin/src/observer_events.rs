use std::time::Duration;

use bevy_ecs::{event::Event, system::Res};
use bevy_time::Time;

use crate::DebugBound;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RxEvent<In, InError>
where
	In: 'static + Sync + Send + DebugBound,
	InError: 'static + Sync + Send + DebugBound,
{
	Next(In),
	Error(InError),
	Complete,
}

/// Used for scheduling, the subscriptions are ticked with this event
/// ? Could be generic over Schedule or something thats associated with the observer
#[derive(Event, Debug, Clone)]
pub struct RxTick {
	pub now: Duration,
	pub delta: Duration,
}

impl RxTick {
	pub fn new(time: &Res<Time>) -> Self {
		Self {
			now: time.elapsed(),
			delta: time.delta(),
		}
	}
}
