use bevy::app::App;
use bevy_ecs::system::{Commands, Query, QueryLens};
use rx_bevy_core::{DropUnsafeSignalContext, SignalContext, Subscriber};
use short_type_name::short_type_name;

use crate::{ContextWithCommands, EntitySubscriber, EntitySubscription};

pub struct CommandContext<'c> {
	subscription_component_query: QueryLens<'c, &'c mut EntitySubscription<'c, Self>>,
	commands: Commands<'c, 'c>,
}

impl<'c> ContextWithCommands<'c> for CommandContext<'c> {
	#[inline]
	fn commands(&mut self) -> &mut Commands<'c, 'c> {
		&mut self.commands
	}
}

impl<'c> CommandContext<'c> {
	pub fn new(
		commands: Commands<'c, 'c>,
		subscription_component_query: QueryLens<'c, &'c mut EntitySubscription<'c, Self>>,
	) -> Self {
		// // SAFETY: it's always only accessible through a reference
		// let commands: Commands<'static, 'static> = unsafe {
		// 	std::mem::transmute::<Commands<'w, 's>, Commands<'static, 'static>>(commands)
		// };
		Self {
			subscription_component_query,
			commands,
		}
	}
}

impl<'c> SignalContext for CommandContext<'c> {
	type DropSafety = DropUnsafeSignalContext;

	type Sharer<Destination>
		= EntitySubscriber<'c, Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self>;

	fn create_context_to_unsubscribe_on_drop() -> Self {
		panic!(
			"{}::get_context_for_drop() was called, but its impossible to do! This is likely due to an unclosed subscription trying to unsubscribe during Drop, which should not happen!",
			short_type_name::<Self>()
		)
	}
}

#[cfg(test)]
mod test_command_context {
	mod test_can_create_valid_system_that_can_create_context {

		use bevy::app::App;

		fn test_app() -> App {
			let mut app = App::new();
			app.add_systems(Update, test_command_context_creating_system);
			app
		}

		fn test_command_context_creating_system<'c>(
			commands: Commands<'c>,
			mut query: Query<&'c mut EntitySubscription<'c, CommandContext<'c>>>,
		) {
			let lens = query.as_query_lens();
			let context = CommandContext::new(commands, lens);
		}

		#[test]
		fn app_can_be_created_with_system() {
			let mut app = test_app();
			let exit = app.run();
		}
	}
}
