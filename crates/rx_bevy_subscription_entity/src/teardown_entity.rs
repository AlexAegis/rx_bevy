use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_context_command::ContextWithCommands;
use rx_bevy_core::{InnerSubscription, SignalContext, SubscriptionLike, Teardown};

pub struct EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	entity: Option<Entity>,
	teardown: InnerSubscription<Context>,
	_phantom_data: PhantomData<Context>,
}

impl<Context> SignalContext for EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	type Context = Context;
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
		self.teardown.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.teardown.add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		// Will panic! But don't worry about it ;)
		Context::get_context_for_drop()
	}
}
