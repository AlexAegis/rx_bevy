use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;
use rx_bevy_ecs_observable_interval::{IntervalObservableComponent, IntervalObservableOptions};

use rx_bevy::prelude::*;
use rx_bevy_plugin::{
	CommandsUnsubscribeExtension, EntityCommandSubscribeExtension, PipeComponent, RelativeEntity,
	RxNext, RxPlugin,
};

/// This test showcases in what order observables execute their observers
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
				despawn_observable.run_if(input_just_pressed(KeyCode::KeyI)),
			),
		)
		.run()
}

fn next_number_observer(next: Trigger<RxNext<String>>, name_query: Query<&Name>, time: Res<Time>) {
	println!(
		"value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn despawn_observable(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	commands
		.entity(example_entities.interval_observable)
		.despawn();
}

fn unsubscribe_from_interval(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subjects_interval_subscription!");
	commands.unsubscribe(example_entities.subjects_interval_subscription);
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	interval_observable: Entity,
	subjects_interval_subscription: Entity,
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

	let string_printer_entity = commands
		.spawn((
			Name::new("String printer Cube"),
			Transform::from_xyz(2.0, 0.0, 4.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		))
		.observe(next_number_observer)
		.id();

	let interval_observable_entity = commands
		.spawn((
			Name::new("IntervalObservable"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			IntervalObservableComponent::new(IntervalObservableOptions {
				duration: Duration::from_secs(1),
				start_on_subscribe: true,
			}),
		))
		.id();

	let mut piped_observable_entity_commands = commands.spawn((
		Name::new("PipeObservable"),
		Transform::from_xyz(-2.0, 0.0, 0.0),
		Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		PipeComponent::new(
			RelativeEntity::Other(interval_observable_entity),
			IdentityOperator::<i32, ()>::default()
				.pipe(map(|i| i * 2))
				.pipe(map(|i| format!("mapped! {i}"))),
		),
	));

	// TODO: Implement "piped subscriptions", where operators are added between the observable and the subscription, like only subscribing for 4 events using skip(4). Add a pipe extension to both commands and entitycommands, and have them return a struct that holds that operator, and also a method to further pipe it or create the subscription
	let subscription = piped_observable_entity_commands
		.subscribe_to_this_scheduled::<String, (), Update>(RelativeEntity::Other(
			string_printer_entity,
		));

	commands.insert_resource(ExampleEntities {
		interval_observable: interval_observable_entity,
		subjects_interval_subscription: subscription,
	});
}
