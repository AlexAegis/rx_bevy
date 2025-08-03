use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};

#[cfg(feature = "debug")]
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{OperatorComponent, SignalBound};

#[derive(Component, Deref, DerefMut)]
#[relationship(relationship_target=OperatorSubscribeObserverRef::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct OperatorSubscribeObserverOf<O>
where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O> OperatorSubscribeObserverOf<O>
where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `OperatorComponent` entity
#[derive(Component, Deref, DerefMut)]
#[relationship_target(relationship=OperatorSubscribeObserverOf::<O>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct OperatorSubscribeObserverRef<O>
where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	#[relationship]
	#[deref]
	subscribe_observer_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O> OperatorSubscribeObserverRef<O>
where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	pub fn new(subscribe_observer_entity: Entity) -> Self {
		Self {
			subscribe_observer_entity,
			_phantom_data: PhantomData,
		}
	}
}
