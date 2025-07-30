use std::{any::TypeId, fmt::Debug, marker::PhantomData};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_erased_component_registry::{
	AppRegisterErasedComponentExtension, EntityCommandInsertErasedComponentByTypeIdExtension,
};
use examples_common::send_event;

/// Press space a few times and see the entities spawned in the world inspector
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.insert_resource(ExampleErasedComponents {
			generic_flag_a: TypeId::of::<GenericFlagComponent<A>>(),
			generic_flag_b: TypeId::of::<GenericFlagComponent<B>>(),
		})
		.register_erased_component::<GenericFlagComponent<A>>()
		.register_erased_component::<GenericFlagComponent<B>>()
		.add_systems(
			Update,
			(
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				spawn_new_random_flag.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn spawn_new_random_flag(
	mut commands: Commands,
	example_erased_components: Res<ExampleErasedComponents>,
) {
	let flag_type_id = if rand::random::<bool>() {
		example_erased_components.generic_flag_a
	} else {
		example_erased_components.generic_flag_b
	};

	commands
		.spawn_empty()
		.insert_erased_component_by_type_id(flag_type_id);
}

#[derive(Component, Default, Debug)]
struct A;

#[derive(Component, Default, Debug)]
struct B;

#[derive(Resource)]
pub struct ExampleErasedComponents {
	generic_flag_a: TypeId,
	generic_flag_b: TypeId,
}

#[derive(Component, Default, Debug)]
pub struct GenericFlagComponent<F> {
	_phantom_data: PhantomData<F>,
}
