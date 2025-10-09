use bevy_ecs::system::Commands;
use rx_bevy_core::{DropUnsafeSignalContext, SignalContext};
use short_type_name::short_type_name;

use crate::ContextWithCommands;

pub struct CommandContext<'c> {
	commands: Commands<'c, 'c>,
}

impl<'c> ContextWithCommands<'c> for CommandContext<'c> {
	#[inline]
	fn commands(&mut self) -> &mut Commands<'c, 'c> {
		&mut self.commands
	}
}

impl<'c> CommandContext<'c> {
	pub fn new(commands: Commands<'c, 'c>) -> Self {
		// // SAFETY: it's always only accessible through a reference
		// let commands: Commands<'static, 'static> = unsafe {
		// 	std::mem::transmute::<Commands<'w, 's>, Commands<'static, 'static>>(commands)
		// };
		Self { commands }
	}
}

impl<'c> SignalContext for CommandContext<'c> {
	type DropSafety = DropUnsafeSignalContext;

	fn create_context_to_unsubscribe_on_drop() -> Self {
		panic!(
			"{}::get_context_for_drop() was called, but its impossible to do! This is likely due to an unclosed subscription trying to unsubscribe during Drop, which should not happen!",
			short_type_name::<Self>()
		)
	}
}
