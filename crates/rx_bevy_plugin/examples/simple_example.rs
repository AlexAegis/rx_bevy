use std::{ops::RangeInclusive, time::Duration};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;
use rx_bevy_ecs_observable_interval::{IntervalObservableComponent, IntervalObservableOptions};

use rx_bevy_plugin::{
	CommandsUnsubscribeExtension, EntityCommandSubscribeExtension, IteratorObservableComponent,
	RelativeEntity, RxNext, RxPlugin, SubjectComponent,
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
	commands.unsubscribe(example_entities.subjects_interval_subscription);
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
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

	let observer_entity = commands
		.spawn((
			Name::new("Other Cube"),
			Transform::from_xyz(-1.0, 0.0, 4.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		))
		.observe(next_number_observer)
		.id();

	let another_observer_entity = commands
		.spawn((
			Name::new("Another Cube"),
			Transform::from_xyz(-1.0, 0.0, 4.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		))
		.observe(next_number_observer)
		.id();

	let mut subject_entity_commands = commands.spawn((
		Name::new("Subject Cube"),
		Transform::from_xyz(2.0, 0.0, 4.0),
		Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		SubjectComponent::<i32, ()>::new(),
	));

	let _s = subject_entity_commands
		.subscribe_to_this_unscheduled::<i32, ()>(RelativeEntity::Other(observer_entity));

	let _s2 = subject_entity_commands
		.subscribe_to_this_unscheduled::<i32, ()>(RelativeEntity::Other(another_observer_entity));

	let subject_entity = subject_entity_commands.id();

	let mut iterator_observable_entity_commands = commands.spawn((
		Name::new("IteratorObservable"),
		Transform::from_xyz(-1.0, 0.0, 0.0),
		Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		IteratorObservableComponent::<RangeInclusive<i32>, true>::new(90..=99),
	));

	let _subjects_iterator_observable_subscription = iterator_observable_entity_commands
		.subscribe_to_this_scheduled::<i32, (), Update>(RelativeEntity::Other(subject_entity));

	// TODO: Add another interval, one should use a virtual clock an the other a real clock
	let mut interval_observable_entity_commands = commands.spawn((
		Name::new("IntervalObservable"),
		Transform::from_xyz(-1.0, 0.0, 0.0),
		Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
		IntervalObservableComponent::new(IntervalObservableOptions {
			duration: Duration::from_secs(1),
			start_on_subscribe: true,
		}),
	));
	// TODO: Implement "piped subscriptions", where operators are added between the observable and the subscription, like only subscribing for 4 events using skip(4)
	let subjects_interval_subscription = interval_observable_entity_commands
		.subscribe_to_this_scheduled::<i32, (), Update>(RelativeEntity::Other(subject_entity));

	commands.insert_resource(ExampleEntities {
		subjects_interval_subscription,
	});
}
