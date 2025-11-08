use std::any::TypeId;

use bevy_ecs::{
	system::{EntityCommand, EntityCommands},
	world::EntityWorldMut,
};
use bevy_ptr::OwningPtr;

use crate::ErasedComponentRegistry;

pub trait EntityCommandInsertErasedComponentByTypeIdExtension {
	/// Requires the component to be registered using
	fn insert_erased_component_by_type_id(&mut self, type_id: TypeId) -> &mut Self;
}

impl EntityCommandInsertErasedComponentByTypeIdExtension for EntityCommands<'_> {
	fn insert_erased_component_by_type_id(&mut self, type_id: TypeId) -> &mut Self {
		self.queue(insert_erased_component_by_type_id(type_id));
		self
	}
}

fn insert_erased_component_by_type_id(type_id: TypeId) -> impl EntityCommand {
	move |mut entity: EntityWorldMut| {
		let type_may_not_be_registered_error_msg = format!(
			"Have you forgot to register {type_id:?} in a plugin using `.register_erased_component::<C>()`?",
		);

		// SAFETY: `update_location` is called at the end, even though no other operations are done to this entity.
		// This command always inserts a new component into an entity.
		let world = unsafe { entity.world_mut() };

		let erased_component_registry = world
			.get_resource::<ErasedComponentRegistry>()
			.unwrap_or_else(|| {
				panic!(
					"DefaultComponentRegistry is not found! {type_may_not_be_registered_error_msg}",
				)
			});

		let erased_subscription_schedule_ctor = erased_component_registry
			.get_constructor(type_id)
			.unwrap_or_else(|| {
				panic!(
					"Component constructor not found in registry! {type_may_not_be_registered_error_msg}",
				)
			});

		let erased_subscription_schedule = erased_subscription_schedule_ctor(world);

		let component_id = world.components().get_id(type_id).unwrap_or_else(|| {
			panic!("ComponentId not found for this TypeId! {type_may_not_be_registered_error_msg}",)
		});

		// SAFETY: ComponentId is extracted from this world, and we would've
		// panicked earlier if it would not have been found.
		// SAFETY: The constructor that creates this component can only be
		// created with the actual type of this component.
		OwningPtr::make(
			erased_subscription_schedule,
			|erased_subscription_schedule| unsafe {
				entity.insert_by_id(component_id, erased_subscription_schedule);
			},
		);

		entity.update_location();
	}
}
