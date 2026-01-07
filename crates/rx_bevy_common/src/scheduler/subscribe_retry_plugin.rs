use bevy_app::{First, Plugin};
use bevy_ecs::{resource::Resource, world::World};
use rx_core_traits::Signal;

use crate::SubscribeCommand;

pub(crate) struct SubscribeRetryPlugin;

impl Plugin for SubscribeRetryPlugin {
	fn build(&self, app: &mut bevy_app::App) {
		app.init_resource::<SubscribesToRetry>();
		app.add_systems(First, execute_pending_retries);
	}
}

#[derive(Resource, Default)]
pub(crate) struct SubscribesToRetry {
	retries: Vec<Box<dyn FnOnce(&mut World) + 'static + Sync + Send>>,
}

impl SubscribesToRetry {
	pub(crate) fn push<Out, OutError>(&mut self, subscribe_command: SubscribeCommand<Out, OutError>)
	where
		Out: Signal,
		OutError: Signal,
	{
		self.retries.push(Box::new(move |world| {
			world.commands().queue(subscribe_command);
		}));
	}

	pub(crate) fn execute(&mut self, world: &mut World) {
		for deferred_subscribe in self.retries.drain(..) {
			deferred_subscribe(world);
		}
	}
}

pub(crate) fn execute_pending_retries(world: &mut World) {
	if let Some(mut subscribes_to_retry) = world.remove_resource::<SubscribesToRetry>() {
		subscribes_to_retry.execute(world);
		world.insert_resource(subscribes_to_retry);
		world.flush();
	}
}
