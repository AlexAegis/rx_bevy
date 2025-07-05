use bevy::{ecs::component::Mutable, prelude::*};
use bevy_ecs::{component::HookContext, world::DeferredWorld};
use rx_bevy::ObservableOutput;
use std::{fmt::Debug, marker::PhantomData};

use crate::SubscriptionComponent;

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
pub trait ObservableComponent: ObservableOutput + Component<Mutability = Mutable>
where
	Self::Out: Send + Sync,
	Self::OutError: Send + Sync,
{
	const CAN_SELF_SUBSCRIBE: bool;

	fn on_insert(&mut self, commands: &mut Commands, context: ObservableOnInsertContext);

	fn on_subscribe(&mut self, commands: &mut Commands, context: ObservableOnSubscribeContext);
}

#[derive(Clone, Copy, Debug)]
pub struct ObservableOnInsertContext {
	/// "This" entity
	pub observable_entity: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct ObservableOnSubscribeContext {
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,
}

pub fn setup_observable_hook<O: ObservableComponent>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: Send + Sync,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	{
		let mut commands = deferred_world.commands();
		let mut entity_commands = commands.entity(hook_context.entity);

		// TODO: For some reason this doesn't work, but it is working in SubjectComponent, is it safe there?
		// entity_commands.insert(
		// 	bevy_ecs::prelude::Observer::new(on_observable_subscribe::<O>)
		// 		.with_entity(hook_context.entity),
		// );

		entity_commands.observe(on_observable_subscribe::<O>);
	}

	{
		let (mut entities, mut commands) = deferred_world.entities_and_commands();
		let mut observable_entity = entities.get_mut(hook_context.entity).unwrap();
		let mut component = observable_entity.get_mut::<O>().unwrap();

		component.on_insert(
			&mut commands,
			ObservableOnInsertContext {
				observable_entity: hook_context.entity,
			},
		);
	}
}

pub fn on_observable_subscribe<O: ObservableComponent>(
	trigger: Trigger<Subscribe<O>>,
	mut observable_component_query: Query<(
		&mut O,
		Option<&mut SubscriptionComponent<O::Out, O::OutError>>,
	)>,
	mut commands: Commands,
) where
	O: Send + Sync,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	let observable_entity = trigger.target();
	let (mut observable_component, existing_subscription) = observable_component_query
		.get_mut(observable_entity)
		.unwrap();
	let subscriber_entity = trigger.subscriber_entity.resolve(observable_entity);

	if !O::CAN_SELF_SUBSCRIBE && observable_entity == subscriber_entity {
		warn!(
			"Tried to subscribe to itself when it is disallowed! {:?}",
			observable_entity
		);
		return;
	}

	observable_component.on_subscribe(
		&mut commands,
		ObservableOnSubscribeContext {
			observable_entity,
			subscriber_entity,
		},
	);

	if let Some(mut existing_subscription) = existing_subscription {
		existing_subscription.add(subscriber_entity);
	} else {
		commands.entity(observable_entity).insert(
			SubscriptionComponent::<O::Out, O::OutError>::new(subscriber_entity),
		);
	}
}

#[derive(Debug)]
pub enum SubscriberEntity {
	This,
	Other(Entity),
}

impl SubscriberEntity {
	pub fn resolve(&self, observable_entity: Entity) -> Entity {
		match self {
			Self::Other(entity) => *entity,
			Self::This => observable_entity,
		}
	}
}

#[derive(Event, Debug)]
pub struct Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	pub subscriber_entity: SubscriberEntity,
	pub _phantom_data: PhantomData<O>,
}

impl<O> Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	pub fn new(subscriber_entity: SubscriberEntity) -> Self {
		Self {
			subscriber_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<O> From<SubscriberEntity> for Subscribe<O>
where
	O: ObservableComponent,
	O::Out: Send + Sync,
	O::OutError: Send + Sync,
{
	fn from(subscriber_entity: SubscriberEntity) -> Self {
		Self::new(subscriber_entity)
	}
}
