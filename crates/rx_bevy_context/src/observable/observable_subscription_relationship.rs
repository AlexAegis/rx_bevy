use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{component::Component, entity::Entity};
use rx_core_traits::Observable;

use core::marker::PhantomData;
#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::RxBevyContext;

/// Stores the reference to the observer entity handling `Subscribe` events
/// for an `ObservableComponent` entity
#[derive(Component, Deref, DerefMut)]
#[relationship_target(relationship=SubscriptionOf::<O>, linked_spawn)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ObservableSubscriptions<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	#[relationship]
	#[deref]
	subscriptions: Vec<Entity>,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O> ObservableSubscriptions<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	pub fn get_subscription_entities(&self) -> Vec<Entity> {
		self.subscriptions.clone()
	}
}

impl<O> Default for ObservableSubscriptions<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	fn default() -> Self {
		Self {
			subscriptions: Vec::new(),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(Component, Deref, DerefMut)]
#[relationship(relationship_target=ObservableSubscriptions::<O>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionOf<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
{
	#[relationship]
	#[deref]
	observable_entity: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<O>,
}

impl<O> SubscriptionOf<O>
where
	O: 'static + Observable<Context = RxBevyContext> + Send + Sync,
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
