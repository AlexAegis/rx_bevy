use bevy_ecs::{
	entity::Entity, event::EntityEvent, hierarchy::ChildOf, schedule::ScheduleLabel,
	system::Commands,
};
use bevy_mod_erased_component_registry::EntityCommandInsertErasedComponentByTypeIdExtension;
use core::marker::PhantomData;
use rx_core_traits::{SignalBound, Subscriber, UpgradeableObserver};
use std::any::TypeId;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{BevySubscriptionContextProvider, SubscriptionSchedule};

/// The destination is erased so observers can listen to this event based on
/// the observables output types only.
#[derive(EntityEvent)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	/// From which entity should the subscription be created from.
	#[event_target]
	pub(crate) observable_entity: Entity,
	/// To where the subscriptions events should be sent to
	/// The destination must be owned by the subscription, therefore it is
	/// consumed during subscription and a `None` is left in its place.
	/// Therefore you can't trigger a [Subscribe] event on multiple entities
	/// at once, but there isn't an api to do that anyway.
	pub(crate) consumable_destination: Option<
		Box<
			dyn Subscriber<In = Out, InError = OutError, Context = BevySubscriptionContextProvider>,
		>,
	>,
	/// This entity can only be spawned from this events constructors
	pub(crate) subscription_entity: Entity,

	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> Subscribe<Out, OutError>
where
	Out: SignalBound,
	OutError: SignalBound,
{
	pub fn new<Destination, S>(
		observable_entity: Entity,
		destination: Destination,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		S: ScheduleLabel,
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands
			.spawn((
				ChildOf(observable_entity),
				SubscriptionSchedule::<S>::default(),
			))
			.id();
		println!("spawned subscription {}", subscription_entity);
		(
			Self {
				observable_entity,
				consumable_destination: Some(Box::new(destination.upgrade())),
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn new_with_erased_schedule<Destination>(
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands
			.spawn_empty()
			.insert_component_by_type_id(schedule_component_type_id)
			.id();

		(
			Self {
				observable_entity,
				consumable_destination: Some(Box::new(destination.upgrade())),
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn new_unscheduled<Destination>(
		observable_entity: Entity,
		destination: Destination,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		Destination: 'static
			+ UpgradeableObserver<
				In = Out,
				InError = OutError,
				Context = BevySubscriptionContextProvider,
			>,
	{
		let subscription_entity = commands.spawn_empty().id();

		(
			Self {
				observable_entity,
				consumable_destination: Some(Box::new(destination.upgrade())),
				subscription_entity,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub(crate) fn try_consume_destination(
		&mut self,
	) -> Option<
		Box<
			dyn Subscriber<In = Out, InError = OutError, Context = BevySubscriptionContextProvider>,
		>,
	> {
		self.consumable_destination.take()
	}
}
