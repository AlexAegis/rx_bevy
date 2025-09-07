use bevy_ecs::system::Commands;

pub struct CommandContext<'w, 's> {
	pub commands: Commands<'w, 's>,
}

impl<'w, 's> CommandContext<'w, 's> {
	pub fn new(commands: Commands<'w, 's>) -> Self {
		Self { commands }
	}
}
