use bevy::{ecs::observer::TriggerTargets, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

/// This test showcases in what order observables execute their observers
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.add_systems(Startup, setup)
		.add_systems(Update, fire_propagation_on_space)
		.run()
}

#[derive(Resource)]
struct ExampleEntities {
	root: Entity,
}

fn setup(mut commands: Commands) {
	let root_entity = commands
		.spawn((Name::new("E1"),))
		.observe(handle_event)
		.with_children(|a| {
			a.spawn((Name::new("E2"),))
				.observe(handle_event)
				.with_children(|b| {
					b.spawn((Name::new("E4"),)).observe(handle_event);
				});
			a.spawn((Name::new("E3"),))
				.observe(handle_event)
				.with_children(|b| {
					b.spawn((Name::new("E5"),)).observe(handle_event);
				});
		})
		.id();

	commands.insert_resource(ExampleEntities { root: root_entity });
}

#[derive(Event, Default)]
struct PropagatedEvent;

fn handle_event(
	trigger: Trigger<PropagatedEvent>,
	mut commands: Commands,
	name_query: Query<&Name>,
	children_query: Query<&Children>,
) {
	let trigger_target_name = name_query
		.get(trigger.target())
		.map(|n| n.as_str())
		.unwrap_or("unnamed");

	println!("event triggered for name: {}", trigger_target_name);

	for children in children_query.get(trigger.target()).iter() {
		commands.trigger_targets(
			PropagatedEvent::default(),
			children.entities().collect::<Vec<_>>(),
		);
	}

	println!(
		"after propagation triggered by name: {}",
		trigger_target_name
	);
}

fn fire_propagation_on_space(
	mut commands: Commands,
	keys: Res<ButtonInput<KeyCode>>,
	example_entities: Res<ExampleEntities>,
) {
	if keys.just_pressed(KeyCode::Space) {
		commands.trigger_targets(PropagatedEvent::default(), example_entities.root);
	}
}
