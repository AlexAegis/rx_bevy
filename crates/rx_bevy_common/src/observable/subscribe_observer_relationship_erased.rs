use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};
use rx_core_common::{PhantomInvariant, Signal};

use core::marker::PhantomData;

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, DerefMut, Debug)]
#[relationship_target(relationship=ErasedSubscribeObserverOf::<Out, OutError>)]
pub struct ErasedSubscribeObservers<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[relationship]
	#[deref]
	subscribe_observer_entity: Vec<Entity>,
	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

#[derive(Component, Deref, Debug)]
#[relationship(relationship_target=ErasedSubscribeObservers::<Out, OutError>)]
pub struct ErasedSubscribeObserverOf<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

impl<Out, OutError> ErasedSubscribeObserverOf<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
