use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{
	Action, ActionApp, ActionEvent, ActionPlugin, ActionSocket, IdentitySignalTransformer,
	SignalEventBool, SocketConnections, SocketConnector, SocketConnectorPlugin,
	SocketConnectorSource,
};
use examples_common::send_event;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Select a target entity using number 1, 2, 3 or clear it with space!
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.register_type::<ExampleTargets>()
		.register_action::<ExampleDiscreteMoveAction>()
		.add_plugins((
			ActionPlugin,
			SocketConnectorPlugin::<
				Virtual,
				KeyCode,
				ExampleDiscreteMoveAction,
				IdentitySignalTransformer<bool>,
			>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
		)
		.add_systems(Update, (swap_target, gizmo_to_target))
		.run()
}

#[derive(Component, Reflect, Debug)]
struct Player;

#[derive(Resource, Debug, Reflect)]
struct ExampleTargets {
	target_1: Entity,
	target_2: Entity,
	target_3: Entity,
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

	let mut socket_connector = SocketConnector::<
		KeyCode,
		ExampleDiscreteMoveAction,
		IdentitySignalTransformer<bool>,
	>::default();

	socket_connector
		.action_map
		.insert(KeyCode::KeyW, ExampleDiscreteMoveAction::Up);
	socket_connector
		.action_map
		.insert(KeyCode::KeyA, ExampleDiscreteMoveAction::Left);
	socket_connector
		.action_map
		.insert(KeyCode::KeyS, ExampleDiscreteMoveAction::Down);
	socket_connector
		.action_map
		.insert(KeyCode::KeyD, ExampleDiscreteMoveAction::Right);

	commands.spawn((
		Player,
		Name::new("player"),
		Transform::from_xyz(0.0, 0.0, 2.0),
		Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
		socket_connector,
	));

	let target_1 = commands
		.spawn((
			Name::new("target 1"),
			Transform::from_xyz(-1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			ActionSocket::<ExampleDiscreteMoveAction>::default(),
		))
		.observe(move_action_observer)
		.id();

	let target_2 = commands
		.spawn((
			Name::new("target 2"),
			Transform::from_xyz(0.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			ActionSocket::<ExampleDiscreteMoveAction>::default(),
		))
		.observe(move_action_observer)
		.id();

	let target_3 = commands
		.spawn((
			Name::new("target 3"),
			Transform::from_xyz(1.0, 0.0, 0.0),
			Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::srgb(0.3, 0.3, 0.9)))),
			ActionSocket::<ExampleDiscreteMoveAction>::default(),
		))
		.observe(move_action_observer)
		.id();

	commands.insert_resource(ExampleTargets {
		target_1,
		target_2,
		target_3,
	});
}

fn move_action_observer(
	trigger: Trigger<ActionEvent<ExampleDiscreteMoveAction>>,
	mut transform_query: Query<&mut Transform>,
) {
	println!("target 3 event! {:?}", trigger);
	if matches!(trigger.event().event, SignalEventBool::Activated) {
		let direction = match trigger.event().action {
			ExampleDiscreteMoveAction::Up => -Vec3::Z,
			ExampleDiscreteMoveAction::Down => Vec3::Z,
			ExampleDiscreteMoveAction::Left => -Vec3::X,
			ExampleDiscreteMoveAction::Right => Vec3::X,
		};

		if let Ok(mut transform) = transform_query.get_mut(trigger.target()) {
			transform.translation += direction * 0.05;
		}
	}
}

fn gizmo_to_target(
	mut gizmos: Gizmos,
	player_query: Query<(Entity, &SocketConnectorSource<ExampleDiscreteMoveAction>), With<Player>>,
	transform_query: Query<&GlobalTransform>,
) {
	for (player_entity, connector_source) in player_query.iter() {
		let from = transform_query.get(player_entity).unwrap();
		let to = transform_query.get(connector_source.entity()).unwrap();

		gizmos.arrow(
			from.translation(),
			to.translation(),
			Color::srgb(0.8, 0.8, 0.1),
		);
	}
}

/// For the sake of simplicity, the target swapping itself is not done by signals
fn swap_target(
	key_presses: Res<ButtonInput<KeyCode>>,
	player_query: Query<Entity, With<Player>>,
	mut commands: Commands,
	example_targets: Res<ExampleTargets>,
) {
	let target = if key_presses.just_pressed(KeyCode::Digit1) {
		Some(example_targets.target_1)
	} else if key_presses.just_pressed(KeyCode::Digit2) {
		Some(example_targets.target_2)
	} else if key_presses.just_pressed(KeyCode::Digit3) {
		Some(example_targets.target_3)
	} else {
		None
	};

	for player_entity in player_query.iter() {
		if let Some(target) = target {
			commands.entity(target).insert(
				SocketConnectorSource::<ExampleDiscreteMoveAction>::new(player_entity),
			);
		}

		if key_presses.just_pressed(KeyCode::Space) {
			commands
				.entity(player_entity)
				.remove::<SocketConnections<ExampleDiscreteMoveAction>>();
		}
	}
}

/// Every time this action is fired, it moves the target's translate a unit
/// on the XY plane
#[derive(Event, Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
enum ExampleDiscreteMoveAction {
	Up,
	Right,
	Down,
	Left,
}

impl Action for ExampleDiscreteMoveAction {
	type Signal = bool;
}
