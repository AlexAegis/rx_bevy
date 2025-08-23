use std::ops::RangeInclusive;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;

use rx_bevy_ecs_observable_iterator::IteratorObservableComponent;
use rx_bevy_plugin::{
	CommandsUnsubscribeExtension, EntityCommandSubscribeExtension, RelativeEntity, RxNext, RxPlugin,
};

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				unsubscribe_from_interval.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn next_number_observer(next: Trigger<RxNext<i32>>, name_query: Query<&Name>, time: Res<Time>) {
	println!(
		"value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn unsubscribe_from_interval(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subjects_interval_subscription!");
	commands.unsubscribe(example_entities.subscription);
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	subscription: Entity,
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let mut iterator_observable_entity_commands = commands.spawn((
		Name::new("IteratorObservable"),
		Transform::from_xyz(-1.0, 0.0, 0.0),
		Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		IteratorObservableComponent::<RangeInclusive<i32>, true>::new(90..=99),
	));

	iterator_observable_entity_commands.observe(next_number_observer);

	let iterator_observable_subscription = iterator_observable_entity_commands
		.subscribe_to_this_scheduled::<i32, (), Update>(RelativeEntity::This);

	commands.insert_resource(ExampleEntities {
		subscription: iterator_observable_subscription,
	});
}
