use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kit_action::{Action, ActionContext, ActionMapPlugin, ActionPlugin, ActionStart};

fn main() -> AppExit {
	App::new()
		.add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
		.add_plugins(ActionPlugin)
		.add_plugins(ActionMapPlugin::<KeyCode, ExampleFireAction>::default())
		.add_systems(Startup, setup)
		.add_systems(Update, trigger_actions)
		.run()
}

#[derive(Resource)]
struct ExampleEntities {
	parent: Entity,
	child: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(4., 4., 10.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let child = {
		let mut child = commands.spawn((
			Name::new("child"),
			ActionContext::<ExampleFireAction>::default(),
		));
		child.observe(handle_fire_action);
		child.id()
	};

	let parent = {
		let mut parent = commands.spawn(Name::new("parent"));
		parent.observe(handle_fire_action);
		parent.observe(handle_discrete_move_action);
		parent.add_child(child);
		parent.id()
	};

	commands.insert_resource(ExampleEntities { child, parent });
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

#[derive(Event, Clone, Copy, Debug, Eq, PartialEq, Hash, Reflect)]
struct ExampleFireAction;

/// NewTypes for action data conversion
#[derive(Debug, Default, Reflect)]
struct ExampleFireSignal(u8);

impl Action for ExampleFireAction {
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

fn handle_fire_action(trigger: Trigger<ActionStart<ExampleFireAction>>, name_query: Query<&Name>) {
	println!(
		"test_event triggered {:?} on {:?} name: {}",
		trigger.event(),
		trigger.entity(),
		name_query.get(trigger.entity()).unwrap_or(&Name::default())
	);
}

fn trigger_actions(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mut commands: Commands,
	entities: Res<ExampleEntities>,
) {
	if keyboard_input.just_pressed(KeyCode::Digit1) {
		commands.trigger_targets(ExampleFireAction, entities.parent);
	} else if keyboard_input.just_pressed(KeyCode::Digit2) {
		commands.trigger_targets(ExampleFireAction, entities.child);
	}
}
