use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{
	Action, ActionApp, ActionEvent, ActionPlugin, ActionSocket, AdsrEnvelope, AdsrSignal,
	AdsrSignalTransformer, SocketConnector, SocketConnectorPlugin,
};
use examples_common::send_event;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Simple mapping example
/// TODO: what about socketed keycode actions
///
/// TODO: Implement action mapping from multiple to one, like WASD to Vec2, maybe a simple additive aggregator would be enough and no new KIND of things would need to be implemented, ofc the Vec2 aggregator needs to be finished
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.register_action::<ExampleAdsrMoveAction>()
		.add_plugins(
			(
				ActionPlugin,
				SocketConnectorPlugin::<
					Virtual,
					KeyCode,
					ExampleAdsrMoveAction,
					AdsrSignalTransformer,
				>::default(),
			),
		)
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
		SocketConnector::<KeyCode, ExampleAdsrMoveAction, AdsrSignalTransformer>::new(|| {
			AdsrSignalTransformer::new(AdsrEnvelope {
				attack_time: Duration::from_millis(400),
				attack_easing: Some(EaseFunction::CubicOut),
				decay_time: Duration::from_millis(200),
				decay_easing: Some(EaseFunction::BackInOut),
				release_time: Duration::from_millis(800),
				release_easing: Some(EaseFunction::Linear),
				sustain_volume: 0.6,
			})
		});
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

	commands
		.spawn((
			Name::new("target"),
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
			socket_connector,
			ActionSocket::<ExampleAdsrMoveAction>::default(),
		))
		.observe(observe_adsr_events);
}

fn observe_adsr_events(trigger: Trigger<ActionEvent<ExampleAdsrMoveAction>>) {
	println!("trigger.event().event {:?}", trigger.event().event);
}

/// Every time this action is fired, it moves the target's translate a unit
/// on the XY plane
#[derive(Event, Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
enum ExampleAdsrMoveAction {
	Up,
	Right,
	Down,
	Left,
}

impl Action for ExampleAdsrMoveAction {
	type Signal = AdsrSignal;
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

			transform.translation += direction * 0.05 * signal.value; // TODO: account for signal strength, and not on fire
		}
	}
}
