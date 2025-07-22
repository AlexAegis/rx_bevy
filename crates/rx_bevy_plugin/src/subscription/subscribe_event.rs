use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event, schedule::ScheduleLabel, system::Commands};
use bevy_log::error;

use thiserror::Error;

use crate::{ObservableSignalBound, RelativeEntity, SubscriptionSchedule};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscribe<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	subscriber_entity: RelativeEntity,
	/// This entity can only be spawned from this events constructors
	subscription_entity: Entity,
	scheduled: bool,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	pub fn get_subscriber_entity_or_this(&self, or_another: Entity) -> Entity {
		self.subscriber_entity.this_or(or_another)
	}

	/// Be aware that if you can't subscribe to a scheduled observable
	/// with an unscheduled subscribe request
	pub fn unscheduled(
		subscriber_entity: RelativeEntity,
		commands: &mut Commands,
	) -> (Self, Entity) {
		let subscription_entity = commands.spawn_empty().id();

		(
			Self {
				subscriber_entity,
				subscription_entity,
				scheduled: false,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn scheduled<S>(
		subscriber_entity: RelativeEntity,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		S: ScheduleLabel,
	{
		let subscription_entity = commands.spawn(SubscriptionSchedule::<S>::default()).id();

		(
			Self {
				subscriber_entity,
				subscription_entity,
				scheduled: true,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn is_scheduled(&self) -> bool {
		self.scheduled
	}

	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}
}

/// TODO: Currently unused, could be used once bevy observers become fallible
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to an entity that does not contain an ObservableComponent")]
	NotAnObservable,
	#[error(
		"Tried to subscribe to an ObservableComponent which disallows subscriptions from the same entity"
	)]
	SelfSubscribeDisallowed,
	#[error("Tried to subscribe to a scheduled observable with an unscheduled Subscription!")]
	UnscheduledSubscribeOnScheduledObservable,
}
