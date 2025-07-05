use bevy::{ecs::component::Mutable, prelude::*};
use rx_bevy::SubscriptionLike;

use crate::RxBufferedSubscriber;

pub trait SubscriptionComponentLike: Component<Mutability = Mutable> {
	fn is_closed(&self) -> bool;

	fn unsubscribe(&mut self);

	fn flush(&mut self, commands: &mut Commands) -> bool;
}

// TODO: This should be able to hold multiple subscriptions, for multiple observing entities, instead of just one
#[derive(Component, Debug, Reflect)]
pub struct SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	// TODO: Teardown events
	subscriber: RxBufferedSubscriber<In, InError>,
}

impl<In, InError> SubscriptionComponentLike for SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.subscriber.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.subscriber.unsubscribe();
	}

	fn flush(&mut self, commands: &mut Commands) -> bool {
		self.subscriber.flush(commands)
	}
}

impl<In, InError> SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	pub fn new(subscriber: RxBufferedSubscriber<In, InError>) -> Self {
		Self { subscriber }
	}
}

pub fn flush_subscriptions_system<C: SubscriptionComponentLike>(
	mut subscription_query: Query<&mut C>,
	mut commands: Commands,
) {
	for mut subscription in subscription_query.iter_mut() {
		subscription.flush(&mut commands);
	}
}
