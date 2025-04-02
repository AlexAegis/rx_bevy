use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{
	Action, ActionMap, ActionMapPlugin, ActionPlugin, ActionSocket, IdentityConverter,
	KeyboardInputSocket,
};

/// Simple mapping example
/// TODO: what about socketed keycode actions
fn main() -> AppExit {
	App::new()
		.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
		.add_plugins((
			ActionPlugin,
			ActionMapPlugin::<KeyCode, ExampleDiscreteMoveAction, IdentityConverter>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(Update, directly_handle_discrete_move_action)
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

	let mut action_map = ActionMap::<KeyCode, ExampleDiscreteMoveAction>::default();
	action_map.insert(KeyCode::KeyW, ExampleDiscreteMoveAction::Up);
	action_map.insert(KeyCode::KeyA, ExampleDiscreteMoveAction::Left);
	action_map.insert(KeyCode::KeyS, ExampleDiscreteMoveAction::Down);
	action_map.insert(KeyCode::KeyD, ExampleDiscreteMoveAction::Right);

	commands.spawn((
		Name::new("target"),
		Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
		MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
		KeyboardInputSocket::default(),
		action_map, // ? how do we know what data to read, and to write
		ActionSocket::<ExampleDiscreteMoveAction>::default(),
	));
}

/// Every time this action is fired, it moves the target's translate a unit
/// on the XY plane
#[derive(Event, Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
enum ExampleDiscreteMoveAction {
	Up,
	Right,
	Down,
	Left,
}

impl Action for ExampleDiscreteMoveAction {
	type Signal = bool;
}

fn directly_handle_discrete_move_action(
	mut action_socket_query: Query<(&mut Transform, &ActionSocket<ExampleDiscreteMoveAction>)>,
) {
	for (mut transform, action_socket) in action_socket_query.iter_mut() {
		for (action, _state) in action_socket.iter().filter(|(_, state)| **state) {
			let direction = match action {
				ExampleDiscreteMoveAction::Up => -Vec3::Z,
				ExampleDiscreteMoveAction::Down => Vec3::Z,
				ExampleDiscreteMoveAction::Left => -Vec3::X,
				ExampleDiscreteMoveAction::Right => Vec3::X,
			};
			transform.translation += direction * 0.05;
		}
	}
}
