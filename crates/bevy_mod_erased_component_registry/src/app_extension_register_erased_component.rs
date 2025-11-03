use bevy_app::App;
use bevy_ecs::{component::Component, world::FromWorld};

use crate::plugin::ErasedComponentRegistryPlugin;

pub trait AppRegisterErasedComponentExtension {
	/// Registering a component as an erased component enables using the
	/// `insert_default_component_by_type_id` EntityCommand for this component.
	/// You can the save a [TypeId] somewhere and insert new [Default] versions
	/// of it from systems that do not know the actual type of that component.
	///
	/// The registration happens on `Startup`, so plugin order may matter if
	/// you also want to interact with it at `Startup`.
	fn register_erased_component<C>(&mut self) -> &mut Self
	where
		C: Component + FromWorld + Send + Sync + 'static;
}

impl AppRegisterErasedComponentExtension for App {
	fn register_erased_component<C>(&mut self) -> &mut Self
	where
		C: Component + FromWorld + Send + Sync + 'static,
	{
		self.add_plugins(ErasedComponentRegistryPlugin::<C>::default())
	}
}
