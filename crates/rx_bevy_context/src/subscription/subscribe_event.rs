use core::marker::PhantomData;
use std::any::TypeId;

use bevy_ecs::{entity::Entity, event::Event, schedule::ScheduleLabel, system::Commands};
use bevy_mod_erased_component_registry::EntityCommandInsertErasedComponentByTypeIdExtension;
use derive_where::derive_where;
use rx_core_traits::SignalBound;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::SubscriptionSchedule;

#[derive(Event, Clone)]
#[derive_where(Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	/// From which entity should the subscription be created from.
	/// TODO (bevy-0.17): while this is not actually needed currently as you could just use the event target, it will be needed in 0.17
	pub(crate) observable_entity: Entity,
	/// To where the subscriptions events should be sent to
	pub(crate) destination_entity: Entity,
	/// This entity can only be spawned from this events constructors
	pub(crate) subscription_entity: Entity,

	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn new<S>(
		observable_entity: Entity,
		destination_entity: Entity,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		S: ScheduleLabel,
	{
		let subscription_entity = commands.spawn(SubscriptionSchedule::<S>::default()).id();

		(
			Self {
				observable_entity,
				destination_entity,
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn new_with_erased_schedule(
		observable_entity: Entity,
		destination_entity: Entity,
		schedule_component_type_id: TypeId,
		commands: &mut Commands,
	) -> (Self, Entity) {
		let subscription_entity = commands
			.spawn_empty()
			.insert_erased_component_by_type_id(schedule_component_type_id)
			.id();

		(
			Self {
				observable_entity,
				destination_entity,
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}
}
