use bevy_derive::Deref;
use bevy_ecs::{component::Component, entity::Entity};
use rx_core_common::{Observable, PhantomInvariant};

use core::marker::PhantomData;
#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref)]
#[relationship_target(relationship=SubscriptionOf::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ObservableSubscriptions<O>
where
	O: 'static + Observable + Send + Sync,
{
	#[relationship]
	#[deref]
	subscriptions: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

impl<O> ObservableSubscriptions<O>
where
	O: 'static + Observable + Send + Sync,
{
	pub fn get_subscription_entities(&self) -> Vec<Entity> {
		self.subscriptions.clone()
	}
}

impl<O> Default for ObservableSubscriptions<O>
where
	O: 'static + Observable + Send + Sync,
{
	fn default() -> Self {
		Self {
			subscriptions: Vec::new(),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(Component, Deref)]
#[relationship(relationship_target=ObservableSubscriptions::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionOf<O>
where
	O: 'static + Observable + Send + Sync,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomInvariant<O>,
}

impl<O> SubscriptionOf<O>
where
	O: 'static + Observable + Send + Sync,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}

	pub fn get_observable_entity(&self) -> Entity {
		self.observable_entity
	}
}
