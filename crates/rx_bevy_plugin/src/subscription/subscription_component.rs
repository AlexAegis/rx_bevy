use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_ecs::{
	component::HookContext, relationship::RelationshipSourceCollection, world::DeferredWorld,
};
use derive_where::derive_where;
use smallvec::{SmallVec, smallvec};

use crate::{
	ObservableComponent, ObservableSignalBound, RxTick, ScheduledSubscription, SubscriptionContext,
	SubscriptionEntityContext, SubscriptionMarkerComponent,
};

/// This semantically is a relationship but that imposes too many restrictions,
/// and subscriptions are managed uniquely anyways.
#[derive(Component)]
#[component(on_remove = subscriptions_on_remove_hook::<O>)]
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
		self.subscriptions.to_vec()
	}

	pub fn get_subscribers(
		&self,
		subscription_query: &Query<&SubscriptionComponent<O>>,
	) -> Vec<Entity> {
		self.subscriptions
			.iter()
			.filter_map(|subscription_entity| {
				subscription_query
					.get(subscription_entity)
					.ok()
					.map(|subscription| subscription.subscriber_entity)
			})
			.collect()
	}
}

/// Replicating relationship behavior by removing it's reference from a subscription component
fn subscriptions_on_remove_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let subscriptions = deferred_world
		.get::<Subscriptions<O>>(hook_context.entity)
		.expect("the component should be available as the hook is for this component")
		.subscriptions
		.clone();

	for subscription_entity in subscriptions.into_iter() {
		if let Some((mut scheduled_subscription, observable_entity, subscriber_entity)) =
			deferred_world
				.get_mut::<SubscriptionComponent<O>>(subscription_entity)
				.map(|mut subscription_component| {
					(
						subscription_component
							.scheduled_subscription
							.take()
							.expect("the subscription has to be present until unsubscribe"),
						subscription_component.observable_entity,
						subscription_component.subscriber_entity,
					)
				}) {
			let mut commands = deferred_world.commands();

			scheduled_subscription.unsubscribe(SubscriptionContext {
				commands: &mut commands,
				observable_entity,
				subscription_entity,
				subscriber_entity,
			});
		}
	}
}

#[derive(Component, Debug)]
#[require(SubscriptionMarkerComponent)] // Erased type to trigger `Tick` events without the knowledge of the actual Observables type
#[component(on_remove = subscription_on_remove_hook::<O>)]
pub struct SubscriptionComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	observable_entity: Entity,
	subscriber_entity: Entity,
	/// This is only an [Option] so it can be removed from the component while it's unsubscribing
	/// Note that it is `None` already when the `unsubscribe` is running, not that you would need
	/// to access it from here, since it's available as `self`
	pub scheduled_subscription: Option<O::ScheduledSubscription>,
	_phantom_data: PhantomData<O>,
}

fn subscription_on_remove_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let subscription_entity = hook_context.entity;

	let (mut scheduled_subscription, entity_context) = {
		let mut subscription = deferred_world
			.get_mut::<SubscriptionComponent<O>>(subscription_entity)
			.expect("the component should be available as the hook is for this component");

		(
			subscription
				.scheduled_subscription
				.take()
				.expect("the subscription has to be present until unsubscribe"),
			subscription.get_subscription_entity_context(subscription_entity),
		)
	};

	if let Some(mut subscriptions_component) =
		deferred_world.get_mut::<Subscriptions<O>>(entity_context.observable_entity)
	{
		subscriptions_component
			.subscriptions
			.retain(|&mut subscription_entity_reference| {
				subscription_entity != subscription_entity_reference
			});
	}

	let mut commands = deferred_world.commands();
	scheduled_subscription.unsubscribe(entity_context.upgrade(&mut commands));
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
			scheduled_subscription: Some(scheduled_subscription),
			_phantom_data: PhantomData,
		}
	}

	pub fn tick(&mut self, event: &RxTick, context: SubscriptionContext) {
		self.scheduled_subscription
			.as_mut()
			.expect("subscriber should always be present when ticked")
			.on_tick(event, context);
	}

	pub fn get_subscription_entity_context(
		&self,
		subscription_entity: Entity,
	) -> SubscriptionEntityContext {
		SubscriptionEntityContext {
			observable_entity: self.observable_entity,
			subscriber_entity: self.subscriber_entity,
			subscription_entity,
		}
	}
}
