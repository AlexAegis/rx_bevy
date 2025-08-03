use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};

#[cfg(feature = "debug")]
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{OperatorComponent, SignalBound};

#[derive(Component, Deref, DerefMut)]
#[relationship(relationship_target=SubscriberSignalObserverRef::<Op>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriberSignalObserverOf<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	#[relationship]
	#[deref]
	subscription_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<Op>,
}

impl<Op> SubscriberSignalObserverOf<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			_phantom_data: PhantomData,
		}
	}
}

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, DerefMut)]
#[relationship_target(relationship=SubscriberSignalObserverOf::<Op>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriberSignalObserverRef<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	#[relationship]
	#[deref]
	signal_observer_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<Op>,
}

impl<Op> SubscriberSignalObserverRef<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	pub fn new(signal_observer_entity: Entity) -> Self {
		Self {
			signal_observer_entity,
			_phantom_data: PhantomData,
		}
	}
}
