use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_context_command::ContextWithCommands;
use rx_bevy_core::{SignalContext, SubscriptionLike};

pub struct EntityTeardown<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	entity: Option<Entity>,
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
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		// Will panic!
		Context::get_context_for_drop()
	}
}
