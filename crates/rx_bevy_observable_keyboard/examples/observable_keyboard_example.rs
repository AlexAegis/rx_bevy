use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_input::keyboard::KeyboardInput;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;
use rx_bevy::prelude::*;

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				unsubscribe.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn next_number_observer(
	next: Trigger<ConsumableSubscriberNotificationEvent<KeyboardInput>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) {
	println!(
		"value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event(),
		next.target(),
		name_query.get(next.target()).unwrap(),
		time.elapsed_secs()
	);
}

fn unsubscribe(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subscription!");
	commands.entity(example_entities.subscription).despawn();
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	subscription: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let keyboard_observable_entity = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::default().into_component(),
		))
		.id();

	let keyboard_event_observer = commands
		.spawn((Name::new("KeyboardObserver"),))
		.observe(next_number_observer)
		.id();

	let subscription = commands.subscribe::<KeyboardInput, (), Update>(
		keyboard_observable_entity,
		keyboard_event_observer,
	);

	commands.insert_resource(ExampleEntities { subscription });
}
