use std::marker::PhantomData;

use bevy_ecs::{
	component::Component, entity::Entity, relationship::RelationshipSourceCollection, system::Query,
};
use derive_where::derive_where;
use smallvec::{SmallVec, smallvec};

use crate::{EntityContext, RxSubscription, SignalBound, SubscriberContext};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Component)]
#[relationship(relationship_target=SubscriptionSignalSources::<Sub>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionSignalDestination<Sub>
where
	Sub: RxSubscription + 'static,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	#[relationship]
	destination: Entity,
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<Sub>,
}

impl<Sub> SubscriptionSignalDestination<Sub>
where
	Sub: RxSubscription + 'static,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(destination: Entity) -> Self {
		Self {
			destination,
			_phantom_data: PhantomData,
		}
	}

	pub fn get_destination(&self) -> Entity {
		self.destination
	}

	pub fn get_subscription_entity_context(
		&self,
		subscription_entity: Entity,
	) -> SubscriberContext<Sub::Out, Sub::OutError> {
		SubscriberContext::new(EntityContext {
			destination_entity: self.destination,
			subscription_entity,
		})
	}
}

/// This semantically is a relationship but that imposes too many restrictions,
/// and subscriptions are managed uniquely anyways.
#[derive(Component)]
#[relationship_target(relationship=SubscriptionSignalDestination::<Sub>, linked_spawn)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionSignalSources<Sub>
where
	Sub: RxSubscription + 'static,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	#[relationship]
	subscriptions: SmallVec<[Entity; 1]>,
	_phantom_data: PhantomData<Sub>,
}

impl<Sub> SubscriptionSignalSources<Sub>
where
	Sub: RxSubscription + 'static,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn new(subscription: Entity) -> Self {
		Self {
			subscriptions: smallvec![subscription],
			_phantom_data: PhantomData,
		}
	}

	pub fn push(&mut self, subscription_entity: Entity) {
		self.subscriptions.push(subscription_entity);
	}

	pub fn contains(&self, subscription: Entity) -> bool {
		self.subscriptions.contains(&subscription)
	}

	pub fn get_subscriptions(&self) -> Vec<Entity> {
		self.subscriptions.to_vec()
	}

	pub fn get_subscribers(
		&self,
		subscription_query: &Query<&SubscriptionSignalDestination<Sub>>,
	) -> Vec<Entity> {
		self.subscriptions
			.iter()
			.filter_map(|subscription_entity| {
				subscription_query
					.get(subscription_entity)
					.ok()
					.map(|subscription| subscription.destination)
			})
			.collect()
	}
}
