use std::fmt::Display;

use bevy::ecs::{entity::Entity, name::Name};

impl<'a> From<(Entity, Option<&'a Name>)> for EntityAndName<'a> {
	fn from(value: (Entity, Option<&'a Name>)) -> Self {
		Self(value.0, value.1)
	}
}

pub struct EntityAndName<'a>(Entity, Option<&'a Name>);

impl Display for EntityAndName<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(name) = self.1 {
			write!(f, "{} ({})", name.as_str(), self.0)
		} else {
			write!(f, "{}", self.0)
		}
	}
}
