use bevy_derive::Deref;
use bevy_ecs::{component::Component, entity::Entity};
use rx_core_common::{PhantomInvariant, RxObserver};

use core::marker::PhantomData;
#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref)]
#[relationship_target(relationship=SignalObserverOf::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SignalObserverRef<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	#[relationship]
	#[deref]
	signal_observer_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

#[derive(Component, Deref)]
#[relationship(relationship_target=SignalObserverRef::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SignalObserverOf<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	#[relationship]
	#[deref]
	signal_observer_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

impl<O> SignalObserverOf<O>
where
	O: 'static + RxObserver + Send + Sync,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			signal_observer_entity: observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
