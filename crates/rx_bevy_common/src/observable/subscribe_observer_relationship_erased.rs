use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};
use rx_core_common::{PhantomInvariant, Signal};

use core::marker::PhantomData;
#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, DerefMut)]
#[relationship_target(relationship=ErasedSubscribeObserverOf::<Out, OutError>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ErasedSubscribeObservers<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[relationship]
	#[deref]
	subscribe_observer_entity: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<(Out, OutError)>,
}

#[derive(Component, Deref)]
#[relationship(relationship_target=ErasedSubscribeObservers::<Out, OutError>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ErasedSubscribeObserverOf<Out, OutError>
where
	Out: Signal,
	OutError: Signal,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
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
