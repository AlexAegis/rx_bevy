use std::marker::PhantomData;

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::{component::Component, system::ResMut, world::FromWorld};
use derive_where::derive_where;

use crate::ErasedComponentRegistry;

#[derive_where(Default)]
pub struct ErasedComponentRegistryPlugin<C>
where
	C: Component + FromWorld + Send + Sync + 'static,
{
	_phantom_data: PhantomData<C>,
}

impl<C> Plugin for ErasedComponentRegistryPlugin<C>
where
	C: Component + FromWorld + Send + Sync + 'static,
{
	fn build(&self, app: &mut App) {
		app.world_mut().register_component::<C>();
		app.add_systems(Startup, register_component_default::<C>);
		app.init_resource::<ErasedComponentRegistry>();
	}
}

fn register_component_default<C>(mut registry: ResMut<ErasedComponentRegistry>)
where
	C: Component + FromWorld + Send + Sync + 'static,
{
	registry.register::<C>();
}
