use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	relationship::RelationshipSourceCollection,
	system::Query,
	world::DeferredWorld,
};
use derive_where::derive_where;
use rx_bevy_observable::Tick;
use smallvec::{SmallVec, smallvec};

use crate::{
	CommandSubscriber, EntityContext, OperatorComponent, RxSubscription, SignalBound,
	SubscriberContext,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

// TODO: Lot of overlap between normal and operator subscriptions, figure something out to reduce duplication along the lines of traits
/// This semantically is a relationship but that imposes too many restrictions,
/// and subscriptions are managed uniquely anyways.
#[derive(Component)]
#[component(on_remove = operator_subscriptions_on_remove_hook::<Op>)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct OperatorSubscriptions<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	subscriptions: SmallVec<[Entity; 1]>,
	_phantom_data: PhantomData<Op>,
}

impl<Op> OperatorSubscriptions<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
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
		subscription_query: &Query<&OperatorSubscriptionComponent<Op>>,
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

/// Replicating relationship behavior by removing it's reference from a subscription component
fn operator_subscriptions_on_remove_hook<O>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	let subscriptions = deferred_world
		.get::<OperatorSubscriptions<O>>(hook_context.entity)
		.expect("the component should be available as the hook is for this component")
		.subscriptions
		.clone();

	for subscription_entity in subscriptions.into_iter() {
		if let Some((mut scheduled_subscription, observable_entity, subscriber_entity)) =
			deferred_world
				.get_mut::<OperatorSubscriptionComponent<O>>(subscription_entity)
				.map(|mut subscription_component| {
					(
						subscription_component
							.subscriber
							.take()
							.expect("the subscription has to be present until unsubscribe"),
						subscription_component.source,
						subscription_component.destination,
					)
				}) {
			let mut commands = deferred_world.commands();

			let context = SubscriberContext::new(EntityContext {
				source_entity: observable_entity,
				destination_entity: subscriber_entity,
				subscription_entity,
			});

			scheduled_subscription.unsubscribe(context.upgrade(&mut commands));
		}
	}
}

#[derive(Component)]
#[component(on_remove = operator_subscription_on_remove_hook::<Op>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct OperatorSubscriptionComponent<Op>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	source: Entity,
	destination: Entity,
	/// This is only an [Option] so it can be removed from the component while it's unsubscribing
	/// Note that it is `None` already when the `unsubscribe` is running, not that you would need
	/// to access it from here, since it's available as `self`
	pub subscriber: Option<Op::Subscriber>,
	_phantom_data: PhantomData<Op>,
}

impl<O> OperatorSubscriptionComponent<O>
where
	O: OperatorComponent + Send + Sync,
	O::In: SignalBound,
	O::InError: SignalBound,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	pub fn new(source: Entity, destination: Entity, subscriber: O::Subscriber) -> Self {
		Self {
			source,
			destination,
			subscriber: Some(subscriber),
			_phantom_data: PhantomData,
		}
	}

	pub fn tick(&mut self, tick: Tick, subscriber: CommandSubscriber<O::Out, O::OutError>) {
		self.subscriber
			.as_mut()
			.expect("subscriber should always be present when ticked")
			.on_tick(tick, subscriber);
	}

	pub fn get_subscription_entity_context(
		&self,
		subscription_entity: Entity,
	) -> SubscriberContext<O::Out, O::OutError> {
		SubscriberContext::new(EntityContext {
			source_entity: self.source,
			destination_entity: self.destination,
			subscription_entity,
		})
	}
}

fn operator_subscription_on_remove_hook<Op>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	let subscription_entity = hook_context.entity;

	let (mut scheduled_subscription, entity_context) = {
		let mut subscription = deferred_world
			.get_mut::<OperatorSubscriptionComponent<Op>>(subscription_entity)
			.expect("the component should be available as the hook is for this component");

		(
			subscription
				.subscriber
				.take()
				.expect("the subscription has to be present until unsubscribe"),
			subscription.get_subscription_entity_context(subscription_entity),
		)
	};

	if let Some(mut subscriptions_component) =
		deferred_world.get_mut::<OperatorSubscriptions<Op>>(entity_context.get_observable_entity())
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
