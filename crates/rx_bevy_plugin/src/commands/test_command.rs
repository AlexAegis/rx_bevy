use bevy_ecs::{entity::Entity, system::Command, world::World};
use rx_core_traits::SignalBound;
use std::marker::PhantomData;

use crate::ObservableComponent;

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

// TODO: Probably useless, maybe as a partial part of the subscribe mechanism, that can flush if needed
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscribeCommand<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	observable: Entity,
	destination: Entity,
	_phantom_data: PhantomData<O>,
}

impl<O> SubscribeCommand<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	pub fn new(observable: Entity, destination: Entity) -> Self {
		Self {
			observable,
			destination,
			_phantom_data: PhantomData,
		}
	}
}

impl<O> Command<Entity> for SubscribeCommand<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	fn apply(self, world: &mut World) -> Entity {
		world.flush();

		Entity::PLACEHOLDER // Return the subscription
	}
}
