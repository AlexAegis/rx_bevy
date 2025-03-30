use bevy::prelude::*;
use bevy_kit_action::{
	ActionContext, ActionKey, ActionMapPlugin, ActionPlugin, ActionStart, KeyboardInputActionData,
};
use bevy_kit_examples_common_3d::ExamplePlugin;

fn main() -> AppExit {
	App::new()
		.add_plugins(ExamplePlugin)
		.add_plugins(ActionPlugin)
		.add_plugins(ActionMapPlugin::<KeyCode, TestAction>::default())
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
	let child = {
		let mut child =
			commands.spawn((Name::new("child"), ActionContext::<TestAction>::default()));
		child.observe(handle_action);
		child.id()
	};

	let parent = {
		let mut parent = commands.spawn(Name::new("parent"));
		parent.observe(handle_action);
		parent.add_child(child);
		parent.id()
	};

	commands.insert_resource(ExampleEntities { child, parent });
}

#[derive(Event, Debug, Eq, PartialEq, Hash, Default, Reflect)]
struct TestAction;

/// NewTypes for action data conversion
#[derive(Debug, Default, Reflect)]
struct TestActionData(u8);

impl ActionKey for TestAction {
	// const DIMENSION: ActionDimension = ActionDimension::;
	type ActionData = TestActionData;
}

impl From<KeyboardInputActionData> for TestActionData {
	fn from(value: KeyboardInputActionData) -> Self {
		Self(if *value { 1 } else { 0 })
	}
}

fn handle_action(trigger: Trigger<ActionStart<TestAction>>, name_query: Query<&Name>) {
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
		commands.trigger_targets(TestAction, entities.parent);
	} else if keyboard_input.just_pressed(KeyCode::Digit2) {
		commands.trigger_targets(TestAction, entities.child);
	}
}
