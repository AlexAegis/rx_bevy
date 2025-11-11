use bevy::{
	input::{common_conditions::input_just_pressed, keyboard::KeyboardInput},
	prelude::*,
};
use bevy_egui::EguiPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
use rx_bevy::prelude::*;
use rx_bevy_context::RxSignal;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			//  EguiPlugin::default(), TODO(bevy-0.17): EguiPlugin::default()
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				unsubscribe.run_if(input_just_pressed(KeyCode::KeyQ)),
			),
		)
		.run()
}

fn next_bool_observer(next: Trigger<RxSignal<bool>>, name_query: Query<&Name>, time: Res<Time>) {
	println!(
		"bool value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn next_keyboard_input_observer(
	next: Trigger<RxSignal<KeyboardInput>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) {
	println!(
		"keyboard_input value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn next_adsr_observer(
	next: Trigger<RxSignal<AdsrSignal>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) {
	println!(
		"adsr value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn unsubscribe(mut _commands: Commands, _example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subscription!");
	//	commands.unsubscribe(example_entities.subscription);
}

#[derive(Resource)]
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

	let mut keyboard_observable_entity_commands = commands.spawn((
		Name::new("KeyboardObservable"),
		KeyboardObservable::default().into_component(),
	));

	keyboard_observable_entity_commands.observe(next_keyboard_input_observer);
	keyboard_observable_entity_commands.observe(next_bool_observer);
	keyboard_observable_entity_commands.observe(next_adsr_observer);

	let keyboard_observable_entity = keyboard_observable_entity_commands.id();

	let target_subscription = commands
		.spawn((
			Name::new("target"),
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
		))
		.observe(handle_move_signal)
		.subscribes_to_observable_entity::<AdsrSignal, (), Update>(keyboard_observable_entity);

	commands.insert_resource(ExampleEntities {
		subscription: target_subscription,
	});
}

fn handle_move_signal(
	next: Trigger<RxSignal<AdsrSignal>>,
	mut transform_query: Query<&mut Transform>,
) {
	if let ObserverNotification::Next(adsr_signal) = next.signal() {
		if let Ok(mut transform) = transform_query.get_mut(next.entity()) {
			transform.translation += Vec3::X * 0.05 * adsr_signal.value;
		}
	}
}
