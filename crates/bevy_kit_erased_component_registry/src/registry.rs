use std::any::{Any, TypeId};

use bevy_ecs::{
	resource::Resource,
	world::{FromWorld, World},
};
use bevy_platform::collections::HashMap;

#[derive(Resource, Default)]
pub struct ErasedComponentRegistry {
	registry: HashMap<TypeId, fn(&mut World) -> Box<dyn Any + Send + Sync>>,
}

impl ErasedComponentRegistry {
	pub fn register<T: FromWorld + Send + Sync + 'static>(&mut self) {
		self.registry
			.insert(TypeId::of::<T>(), |world: &mut World| {
				Box::new(T::from_world(world))
			});
	}

	pub fn get_constructor(
		&self,
		type_id: TypeId,
	) -> Option<&fn(&mut World) -> Box<dyn Any + Send + Sync>> {
		self.registry.get(&type_id)
	}
}
