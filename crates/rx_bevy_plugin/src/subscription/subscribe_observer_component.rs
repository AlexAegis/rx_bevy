use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};

use crate::{ObservableComponent, ObservableSignalBound};

/// TODO: Add on remove hooks to despawn this and the observable component together, the observable should be removed when this is removed, and when the observable is removed this entire entity should despawn
#[derive(Component)]
pub struct SubscribeObserverComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	observable_entity: Entity,
	_phantom_data: PhantomData<O>,
}

impl<O> SubscribeObserverComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
