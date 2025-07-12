use bevy_ecs::entity::Entity;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RelativeEntity {
	This,
	Other(Entity),
}

impl RelativeEntity {
	pub fn this_or(&self, observable_entity: Entity) -> Entity {
		match self {
			Self::Other(entity) => *entity,
			Self::This => observable_entity,
		}
	}
}
