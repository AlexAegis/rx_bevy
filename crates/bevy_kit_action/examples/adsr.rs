use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{
	Action, ActionPlugin, ActionSocket, ActionSocketPlugin, AdsrEnvelope,
	AdsrEnvelopePhaseTransition, AdsrOutputSignal, AdsrSignalTransformer, SocketConnector,
	SocketMapPlugin,
};
use examples_common::send_event;

/// Simple mapping example
/// TODO: what about socketed keycode actions
fn main() -> AppExit {
	App::new()
		.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
		.add_plugins((
			ActionPlugin,
			SocketMapPlugin::<Virtual, KeyCode, ExampleAdsrMoveAction, AdsrSignalTransformer>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.add_systems(Update, handle_adsr_signal_movement)
		.run()
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

	let mut socket_connector =
		SocketConnector::<Virtual, KeyCode, ExampleAdsrMoveAction, AdsrSignalTransformer>::new(
			|| {
				AdsrSignalTransformer::new(AdsrEnvelope {
					attack_time: Duration::from_millis(100),
					release_time: Duration::from_millis(1000),
					..Default::default()
				})
			},
		);
	socket_connector
		.action_map
		.insert(KeyCode::KeyW, ExampleAdsrMoveAction::Up);
	socket_connector
		.action_map
		.insert(KeyCode::KeyA, ExampleAdsrMoveAction::Left);
	socket_connector
		.action_map
		.insert(KeyCode::KeyS, ExampleAdsrMoveAction::Down);
	socket_connector
		.action_map
		.insert(KeyCode::KeyD, ExampleAdsrMoveAction::Right);

	commands.spawn((
		Name::new("target"),
		Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
		ActionSocket::<KeyCode>::default(),
		socket_connector,
		ActionSocket::<ExampleAdsrMoveAction>::default(),
	));
}

/// Every time this action is fired, it moves the target's translate a unit
/// on the XY plane
#[derive(Event, Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
enum ExampleAdsrMoveAction {
	Up,
	Right,
	Down,
	Left,
}

impl Action for ExampleAdsrMoveAction {
	type Signal = AdsrOutputSignal;
}

fn handle_adsr_signal_movement(
	mut action_socket_query: Query<(&mut Transform, &ActionSocket<ExampleAdsrMoveAction>)>,
) {
	for (mut transform, action_socket) in action_socket_query.iter_mut() {
		for (action, signal) in action_socket.iter_signals() {
			let direction = match action {
				ExampleAdsrMoveAction::Up => -Vec3::Z,
				ExampleAdsrMoveAction::Down => Vec3::Z,
				ExampleAdsrMoveAction::Left => -Vec3::X,
				ExampleAdsrMoveAction::Right => Vec3::X,
			};
			let can = match signal.phase_transition {
				Some(AdsrEnvelopePhaseTransition::Fire) => true,
				_ => false,
			};

			if can {
				transform.translation += direction * 0.05; // TODO: account for signal strength, and not on fire
			}
		}
	}
}
