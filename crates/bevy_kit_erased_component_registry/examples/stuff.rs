use bevy::prelude::*;
use bevy_ecs::system::lifetimeless::SCommands;

fn main() -> AppExit {
	App::new().add_systems(Update, foo).run()
}

fn foo(mut commands: Commands) {
	let mut s_commands = unsafe { core::mem::transmute::<Commands, SCommands>(commands) };
	let id = s_commands.spawn_empty().id();
	println!("spawned! {}", id);
}
