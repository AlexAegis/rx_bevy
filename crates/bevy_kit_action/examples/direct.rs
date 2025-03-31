use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{
	Action, ActionContext, ActionMap, ActionPlugin, ActionSocket, ActionStart, KeyboardInputSocket,
};

/// No mapping, just directly interacting with keyboard actions
/// TODO: what about socketed keycode actions
fn main() -> AppExit {
	App::new()
		.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
		.add_plugins(ActionPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, trigger_action_manually)
		.run()
}

#[derive(Resource)]
struct ExampleEntities {
	target: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(4., 4., 10.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let target = {
		let mut action_map = ActionMap::<KeyCode, ExampleDiscreteMoveAction>::default();
		action_map.insert(ExampleDiscreteMoveAction::Up, KeyCode::ArrowUp);
		action_map.insert(ExampleDiscreteMoveAction::Right, KeyCode::ArrowRight);
		action_map.insert(ExampleDiscreteMoveAction::Down, KeyCode::ArrowDown);
		action_map.insert(ExampleDiscreteMoveAction::Left, KeyCode::ArrowLeft);

		let mut entity = commands.spawn((
			Name::new("target"),
			KeyboardInputSocket::default(),
			action_map,
			ActionSocket::<ExampleDiscreteMoveAction, bool>::default(),
		));
		entity.observe(handle_discrete_move_action);

		entity.id()
	};

	commands.insert_resource(ExampleEntities { target });
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
	// const DIMENSION: ActionDimension = ActionDimension::;
	type Signal = bool;
}

fn handle_discrete_move_action(
	trigger: Trigger<ActionStart<ExampleDiscreteMoveAction>>,
	name_query: Query<&Name>,
	mut transform_query: Query<&mut Transform>,
) {
	println!(
		"example_move_action triggered {:?} on {:?} name: {}",
		trigger.event(),
		trigger.entity(),
		name_query.get(trigger.entity()).unwrap_or(&Name::default())
	);

	if let Ok(mut transform) = transform_query.get_mut(trigger.entity()) {
		let direction = match trigger.event().action {
			ExampleDiscreteMoveAction::Up => Vec3::Y,
			ExampleDiscreteMoveAction::Down => -Vec3::Y,
			ExampleDiscreteMoveAction::Left => -Vec3::X,
			ExampleDiscreteMoveAction::Right => Vec3::X,
		};
		transform.translation += direction;
	}
}

fn trigger_action_manually(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	entities: Res<ExampleEntities>,
) {
	if keyboard_input.just_pressed(KeyCode::Digit1) {
		commands.trigger_targets(ExampleDiscreteMoveAction::Left, entities.target);
	} else if keyboard_input.just_pressed(KeyCode::Digit2) {
		commands.trigger_targets(ExampleDiscreteMoveAction::Right, entities.target);
	}
}
