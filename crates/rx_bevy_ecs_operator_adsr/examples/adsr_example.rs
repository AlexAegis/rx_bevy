use std::time::Duration;

use bevy::{
	input::{ButtonState, common_conditions::input_just_pressed, keyboard::KeyboardInput},
	prelude::*,
};
use bevy_egui::EguiPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;
use rx_bevy::MapOperator;
use rx_bevy_ecs_observable_keyboard::{KeyboardObservableComponent, KeyboardObservableOptions};

use rx_bevy_ecs_operator_adsr::{AdsrEnvelope, AdsrOperator, AdsrOperatorOptions, AdsrSignal};
use rx_bevy_plugin::{
	CommandsUnsubscribeExtension, EntityCommandSubscribeExtension, PipeComponent, RelativeEntity,
	RxNext, RxPlugin,
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
				unsubscribe.run_if(input_just_pressed(KeyCode::KeyQ)),
			),
		)
		.run()
}

fn next_bool_observer(next: Trigger<RxNext<bool>>, name_query: Query<&Name>, time: Res<Time>) {
	println!(
		"bool value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn next_keyboard_input_observer(
	next: Trigger<RxNext<KeyboardInput>>,
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
	next: Trigger<RxNext<AdsrSignal>>,
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

fn unsubscribe(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subscription!");
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

	let mut keyboard_observable_entity_commands = commands.spawn((
		Name::new("KeyboardObservable"),
		KeyboardObservableComponent::new(KeyboardObservableOptions {}),
		PipeComponent::new(
			RelativeEntity::This,
			MapOperator::<KeyboardInput, (), _, bool>::new(|input: KeyboardInput| {
				if input.key_code == KeyCode::Space {
					input.state == ButtonState::Pressed
				} else {
					false
				}
			}),
		),
		PipeComponent::new(
			RelativeEntity::This,
			AdsrOperator::<()>::new(AdsrOperatorOptions {
				emit_none_more_than_once: false,
				envelope: AdsrEnvelope {
					attack_time: Duration::from_millis(400),
					attack_easing: Some(EaseFunction::CubicOut),
					decay_time: Duration::from_millis(200),
					decay_easing: Some(EaseFunction::BackInOut),
					release_time: Duration::from_millis(800),
					release_easing: Some(EaseFunction::Linear),
					sustain_volume: 0.6,
				},
			}),
		),
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
		.subscribe_to_that_scheduled::<AdsrSignal, (), Update>(RelativeEntity::Other(
			keyboard_observable_entity,
		));

	commands.insert_resource(ExampleEntities {
		subscription: target_subscription,
	});
}

fn handle_move_signal(
	next: Trigger<RxNext<AdsrSignal>>,
	mut transform_query: Query<&mut Transform>,
) {
	if let Ok(mut transform) = transform_query.get_mut(next.target()) {
		transform.translation += Vec3::X * 0.05 * next.event().value;
	}
}
