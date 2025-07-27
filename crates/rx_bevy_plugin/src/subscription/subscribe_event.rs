use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{
	bundle::Bundle,
	component::{ComponentId, ComponentIdFor, Components},
	entity::Entity,
	event::Event,
	schedule::ScheduleLabel,
	system::Commands,
};
use bevy_log::error;

use thiserror::Error;

use crate::{FlushWorld, ObservableSignalBound, RelativeEntity, SubscriptionSchedule};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event, Clone)]
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
	/// Contains the [TypeId] of a `SubscriptionSchedule::<S>`` component, for
	/// later component cloning while preserving scheduling
	#[reflect(ignore)]
	schedule: Option<TypeId>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: ObservableSignalBound,
	OutError: ObservableSignalBound,
{
	pub fn get_subscriber_entity_or_this(&self, or_another: Entity) -> Entity {
		self.subscriber_entity.or_this(or_another)
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
				schedule: None,
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
				schedule: Some(TypeId::of::<SubscriptionSchedule<S>>()),
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	/// Meant to keep the scheduling of an existing subscription.
	pub fn retarget_existing<NextOut, NextOutError>(
		&self,
		new_subscriber_entity: Entity,
		commands: &mut Commands,
	) -> (Subscribe<NextOut, NextOutError>, Entity)
	where
		NextOut: ObservableSignalBound,
		NextOutError: ObservableSignalBound,
	{
		let subscription_entity = if let Some(subscription_schedule_type_id) = self.schedule {
			dbg!(subscription_schedule_type_id);
			/// TODO: This doesen't work without flushing the world entities can be cloned even if their spawn commands weren't resolved.
			commands.queue(FlushWorld);
			commands
				.entity(self.get_subscription_entity())
				.clone_and_spawn_with(move |builder| {
					builder.deny_all();
					builder.allow_by_type_ids(vec![subscription_schedule_type_id]);
				})
				.id()
		} else {
			commands.spawn_empty().id()
		};

		dbg!(subscription_entity);
		dbg!(new_subscriber_entity);

		(
			Subscribe::<NextOut, NextOutError> {
				subscription_entity,
				subscriber_entity: RelativeEntity::Other(new_subscriber_entity),
				schedule: self.schedule,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn is_scheduled(&self) -> bool {
		self.schedule.is_some()
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
