use std::marker::PhantomData;

use bevy::prelude::*;
use derive_where::derive_where;
use smallvec::{SmallVec, smallvec};

use crate::{
	ObservableComponent, ObservableSignalBound, RxTick, ScheduledSubscription,
	SubscriptionOnTickContext,
};

/// This semantically is a relationship but that imposes too many restrictions,
/// and subscriptions are managed their own way anyways.
#[derive(Component)]
#[derive_where(Default, Debug)]
pub struct Subscriptions<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	subscriptions: SmallVec<[Entity; 1]>,
	_phantom_data: PhantomData<O>,
}

impl<O> Subscriptions<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
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

	pub fn get_subscriptions(&self) -> Vec<Entity> {
		self.subscriptions.iter().copied().collect()
	}

	pub fn get_subscribers(
		&self,
		subscription_query: &Query<&SubscriptionComponent<O>>,
	) -> Vec<Entity> {
		self.subscriptions
			.iter()
			.filter_map(|&subscription_entity| {
				subscription_query
					.get(subscription_entity)
					.ok()
					.map(|subscription| subscription.subscriber_entity)
			})
			.collect()
	}
}

#[derive(Component, Debug)]
// #[relationship(relationship_target = Subscriptions::<O>)]
pub struct SubscriptionComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	// #[relationship]
	observable_entity: Entity,
	subscriber_entity: Entity,
	/// This is only a None while the subscription is set up, will always be Some after that.
	pub scheduled_subscription: O::ScheduledSubscription,
	_phantom_data: PhantomData<O>,
}

impl<O> SubscriptionComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	pub fn new(
		observable_entity: Entity,
		subscriber_entity: Entity,
		scheduled_subscription: O::ScheduledSubscription,
	) -> Self {
		Self {
			observable_entity,
			subscriber_entity,
			scheduled_subscription,
			_phantom_data: PhantomData,
		}
	}

	pub fn tick(&mut self, event: &RxTick, context: SubscriptionOnTickContext) {
		self.scheduled_subscription.on_tick(event, context);
	}

	pub fn into_subscription_on_tick_context<'a, 'w, 's>(
		&self,
		commands: &'a mut Commands<'w, 's>,
		subscription_entity: Entity,
	) -> SubscriptionOnTickContext<'a, 'w, 's> {
		SubscriptionOnTickContext::<'a, 'w, 's> {
			commands,
			observable_entity: self.observable_entity,
			subscriber_entity: self.subscriber_entity,
			subscription_entity,
		}
	}
}
