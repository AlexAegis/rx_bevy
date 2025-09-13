use bevy_ecs::system::Commands;

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

fn goo(mut commands: Commands, ctx: CommandContext) {
	let c = CommandContext::new(commands);
}
