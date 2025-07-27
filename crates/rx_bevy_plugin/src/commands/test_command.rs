use bevy_ecs::{system::Command, world::World};

pub struct FlushWorld;

impl Command for FlushWorld {
	fn apply(self, world: &mut World) -> () {
		println!("FLUSH");
		world.flush();
	}
}
