use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{entity::Entity, event::Event, schedule::ScheduleLabel, system::Commands};
use rx_bevy_core::SignalBound;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::SubscriptionSchedule;

#[derive(Event, Clone)]
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
	// /// Contains the [TypeId] of a `SubscriptionSchedule::<S>` component.
	// /// It is required to erase this type to be able to
	// #[cfg_attr(feature = "reflect", reflect(ignore))]
	// schedule: Option<TypeId>,
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
				// schedule: Some(TypeId::of::<SubscriptionSchedule<S>>()),
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}
}
