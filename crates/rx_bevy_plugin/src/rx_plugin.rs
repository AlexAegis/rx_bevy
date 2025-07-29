use std::{
	any::{Any, TypeId},
	cell::RefCell,
	ptr::NonNull,
	sync::{Arc, RwLock},
};

use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::{
	component::ComponentId,
	entity::Entity,
	resource::Resource,
	system::{EntityCommand, EntityCommands},
	world::EntityWorldMut,
};
use bevy_platform::collections::HashMap;
use bevy_ptr::OwningPtr;
use bevy_reflect::Reflect;
use bevy_time::Virtual;

use crate::RxScheduler;

/// A collection of default plugins
/// TODO: Add a dyn vec of schedules and a chainable .schedule_on method, and the default version adds Update
pub struct RxPlugin;

impl Plugin for RxPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			RxScheduler::<Update, Virtual>::default(),
			RxScheduler::<PostUpdate, Virtual>::default(),
		));
	}
}

#[derive(Resource, Default)]
pub(crate) struct SubscriptionScheduleRegistry {
	registry: HashMap<TypeId, fn() -> Box<dyn Any + Send + Sync>>,
}

impl SubscriptionScheduleRegistry {
	pub fn register<T: Default + Send + Sync + 'static>(&mut self) {
		self.registry
			.insert(TypeId::of::<T>(), || Box::new(T::default()));
	}

	pub fn create(&self, type_id: TypeId) -> Option<Box<dyn Any + Send + Sync>> {
		self.registry.get(&type_id).map(|ctor| ctor())
	}
}

pub trait EntityCommandInsertDefaultComponentByTypeIdExt {
	fn insert_default_component_by_type_id(&mut self, type_id: TypeId) -> &mut Self;
}

impl EntityCommandInsertDefaultComponentByTypeIdExt for EntityCommands<'_> {
	fn insert_default_component_by_type_id(&mut self, type_id: TypeId) -> &mut Self {
		self.queue(insert_default_component_by_type_id(type_id));
		self
	}
}

pub fn insert_default_component_by_type_id(type_id: TypeId) -> impl EntityCommand {
	move |mut entity: EntityWorldMut| {
		let world = unsafe { entity.world_mut() };

		let schedule_label_registry = world
			.get_resource::<SubscriptionScheduleRegistry>()
			.expect("the registry is initialized by the plugin");

		let erased_subscription_schedule_dbg = schedule_label_registry
			.create(type_id)
			.expect("schedule should be registered already");
		dbg!(erased_subscription_schedule_dbg);

		let erased_subscription_schedule = schedule_label_registry
			.create(type_id)
			.expect("schedule should be registered already");

		let component_id = world
			.components()
			.get_id(type_id)
			.expect("it is registered");

		dbg!(component_id);

		OwningPtr::make(erased_subscription_schedule, |asd| unsafe {
			println!("WTF PTR {:?}", asd);
			entity.insert_by_id(component_id, asd);
		});
	}
}
