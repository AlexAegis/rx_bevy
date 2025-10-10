use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable, WithContext};

use crate::ContextWithCommands;

pub struct EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	entity: Option<Entity>,
	subscription: SubscriptionData<Context>,
	_phantom_data: PhantomData<Context>,
}

impl<Context> WithContext for EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	type Context = Context;
}

impl<Context> Tickable for EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.subscription.tick(tick, context);
	}
}

impl<Context> SubscriptionLike for EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.entity.is_none()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Some(entity) = self.entity.take() {
			context.commands().entity(entity).despawn();
		}
		self.subscription.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subscription.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// Will panic! But don't worry about it ;)
		Context::create_context_to_unsubscribe_on_drop()
	}
}
