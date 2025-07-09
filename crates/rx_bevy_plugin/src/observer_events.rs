use std::time::Duration;

use bevy::{
	ecs::event::Event,
	prelude::{Deref, DerefMut},
	time::Time,
};
use bevy_ecs::system::Res;

// TODO: Join these into a single enum if you don't want to spawn 3 of observer entities
#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct RxNext<In>(pub In)
where
	In: 'static + Sync + Send;

#[derive(Event, Deref, DerefMut, Debug, Clone)]
pub struct RxError<InError>(pub InError)
where
	InError: 'static + Sync + Send;

#[derive(Event, Debug, Clone)]
pub struct RxComplete;

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
