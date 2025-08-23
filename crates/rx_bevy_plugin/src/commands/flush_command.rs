use bevy_ecs::{system::Command, world::World};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct FlushCommand;

impl Command for FlushCommand {
	fn apply(self, world: &mut World) {
		world.flush();
	}
}
