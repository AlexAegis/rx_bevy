use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{
	entity::Entity,
	event::Event,
	reflect::{self, ReflectCommandExt},
	schedule::ScheduleLabel,
	system::Commands,
};
use bevy_log::error;

use thiserror::Error;

use crate::{
	EntityCloneFlushAndSpawnedWithExt, EntityCommandInsertDefaultComponentByTypeIdExt,
	RelativeEntity, SignalBound, SubscriptionSchedule,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	subscriber_entity: RelativeEntity,
	/// This entity can only be spawned from this events constructors
	subscription_entity: Entity,
	/// Contains the [TypeId] of a `SubscriptionSchedule::<S>` component, for
	/// later component cloning while preserving scheduling
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	schedule: Option<TypeId>,
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn get_subscriber_entity_or_this(&self, or_this: Entity) -> Entity {
		self.subscriber_entity.or_this(or_this)
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
		NextOut: SignalBound,
		NextOutError: SignalBound,
	{
		let new_subscription_entity = if let Some(subscription_schedule_type_id) = self.schedule {
			dbg!(subscription_schedule_type_id);
			dbg!(self.get_subscription_entity());
			// TODO: This doesn't work in 0.16, but looks like it will in 0.17. Without flushing the world entities can't be cloned if their spawn commands weren't resolved.

			println!(
				"insert_default_component_by_type_id {:?}",
				subscription_schedule_type_id
			);
			let new_subscription_entity = commands
				.spawn_empty()
				.insert_default_component_by_type_id(subscription_schedule_type_id)
				.id();

			// !! simply isnert by typeid through reflection, it is DEFAULT, this whole retarget bs is not needed, if there is no cloning involved

			//commands
			//	.entity(self.get_subscription_entity())
			//	.as_cloned_flushed_and_spawn_with(
			//		new_subscription_entity,
			//		move |builder: &mut bevy_ecs::entity::EntityClonerBuilder| {
			//			//builder.deny_all();
			//			//builder.allow_by_type_ids(vec![subscription_schedule_type_id]);
			//		},
			//	);
			//
			new_subscription_entity
		} else {
			commands.spawn_empty().id()
		};

		dbg!(new_subscription_entity);
		dbg!(new_subscriber_entity);

		(
			Subscribe::<NextOut, NextOutError> {
				subscription_entity: new_subscription_entity,
				subscriber_entity: RelativeEntity::Other(new_subscriber_entity),
				schedule: self.schedule,
				_phantom_data: PhantomData,
			},
			new_subscription_entity,
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
