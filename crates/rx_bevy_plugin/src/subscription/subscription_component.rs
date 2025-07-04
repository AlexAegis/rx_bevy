use bevy::prelude::*;

use crate::CommandQuerySubscriber;

#[derive(Component, Debug, Reflect)]
pub struct SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	// TODO: Teardown events
	subscriber: CommandQuerySubscriber<In, InError>,
}

impl<In, InError> SubscriptionComponent<In, InError>
where
	In: 'static + Send + Sync,
	InError: 'static + Send + Sync,
{
	pub fn new(subscriber: CommandQuerySubscriber<In, InError>) -> Self {
		Self { subscriber }
	}

	pub fn flush(&mut self, commands: &mut Commands) -> bool {
		self.subscriber.flush(commands)
	}
}

// TODO Maybe this trait too will need to be ecs specific, due to the add method
impl<In, InError> rx_bevy::SubscriptionLike for SubscriptionComponent<In, InError>
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

	fn add(&mut self, _subscription: &'static mut dyn rx_bevy::SubscriptionLike) {}
}

pub fn flush_subscriptions_system<In, InError>(
	mut subscription_query: Query<&mut SubscriptionComponent<In, InError>>,
	mut commands: Commands,
) where
	In: Send + Sync + std::fmt::Debug,
	InError: Send + Sync + std::fmt::Debug,
{
	for mut subscription in subscription_query.iter_mut() {
		subscription.flush(&mut commands);
	}
}
