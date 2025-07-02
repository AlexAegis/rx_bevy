use std::any::Any;

use bevy::ecs::entity::Entity;

pub struct EntityObserver<OnNextSystem> {
	pub destination: EntitySubscriptionDestination,
	pub on_next_system: OnNextSystem,
	// TODO: rest of the channels
	// on_error_system: Box<dyn Any + Send + Sync + 'static>,
	// on_complete_system: Box<dyn Any + Send + Sync + 'static>,
	// on_unsubscribe_system: Box<dyn Any + Send + Sync + 'static>,
}

pub struct InternalEntityObserver {
	pub source: Entity,
	pub destination: EntitySubscriptionDestination,
	pub on_next_system: Box<dyn Any + Send + Sync + 'static>,
	// TODO: rest of the channels
	// on_error_system: Box<dyn Any + Send + Sync + 'static>,
	// on_complete_system: Box<dyn Any + Send + Sync + 'static>,
	// on_unsubscribe_system: Box<dyn Any + Send + Sync + 'static>,
}

/// What entity to trigger
pub enum EntitySubscriptionDestination {
	This,
	Other(Entity),
}
