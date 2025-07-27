use bevy_ecs::entity::Entity;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Used for EntityCommands, so the user could refer to the entity being
/// constructed.
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum RelativeEntity {
	This,
	Other(Entity),
}

impl RelativeEntity {
	pub fn or_this(&self, this_entity: Entity) -> Entity {
		match self {
			Self::Other(entity) => *entity,
			Self::This => this_entity,
		}
	}
}
