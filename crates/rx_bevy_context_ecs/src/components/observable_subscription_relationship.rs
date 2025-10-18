use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};
use rx_bevy_core::{Observable, SignalBound};

#[cfg(feature = "debug")]
use std::fmt::Debug;
use std::marker::PhantomData;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{BevySubscriptionContextProvider, EntitySubscriptionContextAccessProvider};

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, DerefMut)]
#[relationship_target(relationship=SubscriptionOf::<O, ContextAccess>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ObservableSubscriptions<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	#[relationship]
	#[deref]
	subscriptions: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O, ContextAccess> Default for ObservableSubscriptions<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn default() -> Self {
		Self {
			subscriptions: Vec::new(),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(Component, Deref, DerefMut)]
#[relationship(relationship_target=ObservableSubscriptions::<O, ContextAccess>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionOf<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O, ContextAccess> SubscriptionOf<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}
