use bevy_ecs::system::Commands;

pub trait ContextWithCommands<'a> {
	fn commands(&mut self) -> &mut Commands<'a, 'a>;
}
