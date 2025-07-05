use std::marker::PhantomData;

use bevy::{ecs::component::Mutable, platform::collections::HashSet, prelude::*};

pub trait SubscriptionComponentLike: Component<Mutability = Mutable> {
	fn is_closed(&self) -> bool;

	fn unsubscribe(&mut self);
}

/// Holds active subscriptions for different destinations
/// TODO: Maybe use markers instead of just In/InError
#[derive(Component, Debug, Reflect)]
pub struct SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	subscribers: HashSet<Entity>,
	closed: bool,
	#[reflect(ignore)]
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	pub fn new(destination: Entity) -> Self {
		let mut subscription = Self::default();
		subscription.subscribers.insert(destination);
		subscription
	}

	pub fn get_subscriber_entities(&self) -> Vec<Entity> {
		self.subscribers.iter().copied().collect()
	}

	pub fn add(&mut self, entity: Entity) {
		self.subscribers.insert(entity);
	}
}

impl<In, InError> SubscriptionComponentLike for SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self) {
		self.closed = true;
	}
}

impl<In, InError> Default for SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn default() -> Self {
		Self {
			subscribers: HashSet::with_capacity(1),
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}
