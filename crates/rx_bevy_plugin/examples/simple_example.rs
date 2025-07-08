use std::{ops::RangeInclusive, time::Duration};

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;

use rx_bevy_plugin::{
	IntervalObservableComponent, IteratorObservableComponent, RxNext, RxPlugin, SubjectComponent,
	SubscribeFor, SubscriberEntity,
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
			send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.run()
}

fn next_number_observer(next: Trigger<RxNext<i32>>, name_query: Query<&Name>) {
	println!(
		"value observed: {:?} by {:?} name: {:?}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap()
	);
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	observable_entity: Entity,
	observer_entity: Entity,
	another_observer_entity: Entity,
	subject_entity: Entity,
	another_observable_entity: Entity,
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

	let subject_entity = commands
		.spawn((
			Name::new("Subject Cube"),
			Transform::from_xyz(2.0, 0.0, 4.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			SubjectComponent::<i32, ()>::new(),
		))
		.observe(next_number_observer)
		.trigger(SubscribeFor::<SubjectComponent<i32, ()>>::new(
			SubscriberEntity::Other(observer_entity),
		))
		.trigger(SubscribeFor::<SubjectComponent<i32, ()>>::new(
			SubscriberEntity::Other(another_observer_entity),
		))
		.id();

	let observable_entity = commands
		.spawn((
			Name::new("IteratorObservable"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			IteratorObservableComponent::new(99..=99),
		))
		.trigger(SubscribeFor::<
			IteratorObservableComponent<RangeInclusive<i32>>,
		>::new(SubscriberEntity::Other(subject_entity)))
		.id();

	let another_observable_entity = commands
		.spawn((
			Name::new("IntervalObservable"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			IntervalObservableComponent::new(Duration::from_secs(1)),
		))
		.trigger(SubscribeFor::<IntervalObservableComponent>::new(
			SubscriberEntity::Other(subject_entity),
		))
		.id();

	println!("spawned");

	commands.insert_resource(ExampleEntities {
		observable_entity,
		observer_entity,
		another_observer_entity,
		subject_entity,
		another_observable_entity,
	});
}
