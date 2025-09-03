use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{entity::Entity, event::Event, schedule::ScheduleLabel, system::Commands};
use bevy_kit_erased_component_registry::EntityCommandInsertErasedComponentByTypeIdExtension;
use rx_bevy_common_bounds::SignalBound;

use crate::{RelativeEntity, SubscriptionSchedule};

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
	destination: RelativeEntity,
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
	pub fn get_destination_or_this(&self, or_this: Entity) -> Entity {
		self.destination.or_this(or_this)
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
				destination: subscriber_entity,
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
				destination: subscriber_entity,
				subscription_entity,
				schedule: Some(TypeId::of::<SubscriptionSchedule<S>>()),
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	/// Creates a new Subscribe event and a new empty Subscription entity.
	/// It is meant to keep the scheduling of an existing subscription.
	/// The event does not need to be fired to be re-targeted.
	///
	/// While this could've been implemented a little bit simpler by requiring
	/// only the TypeId as an argument, requiring an existing Subscribe event
	/// ensures that only Subscribe events can only be created with correct
	/// TypeId's that do actually refer to a `SubscriptionSchedule<S>`
	pub fn new_with_schedule_from<OriginalOut, OriginalOutError>(
		subscriber_entity: Entity,
		use_schedule_from: &Subscribe<OriginalOut, OriginalOutError>,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		OriginalOut: SignalBound,
		OriginalOutError: SignalBound,
	{
		let new_subscription_entity = use_schedule_from.spawn_new_with_this_schedule(commands);

		(
			Self {
				subscription_entity: new_subscription_entity,
				destination: RelativeEntity::Other(subscriber_entity),
				schedule: use_schedule_from.schedule,
				_phantom_data: PhantomData,
			},
			new_subscription_entity,
		)
	}

	/// Spawns a new empty entity to be used to create another Subscription.
	/// If this event was a scheduled subscription, the new event will have
	/// the same [SubscriptionSchedule] component on it.
	pub(crate) fn spawn_new_with_this_schedule(&self, commands: &mut Commands) -> Entity {
		if let Some(subscription_schedule_type_id) = self.schedule {
			commands
				.spawn_empty()
				.insert_erased_component_by_type_id(subscription_schedule_type_id)
				.id()
		} else {
			commands.spawn_empty().id()
		}
	}

	pub fn is_scheduled(&self) -> bool {
		self.schedule.is_some()
	}

	/// The pre-spawned entity that represents the [Subscription] created by this
	/// event
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}
}
