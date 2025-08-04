use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_event;

use rx_bevy_ecs_observable_interval::{IntervalObservableComponent, IntervalObservableOptions};
use rx_bevy_plugin::{
	CommandsUnsubscribeExtension, EntityCommandSubscribeExtension, RelativeEntity, RxPlugin,
	RxSignal,
};

/// This test showcases in what order observables execute their observers
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
				unsubscribe_from_interval.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn next_number_observer(
	next: Trigger<RxSignal<i32, ()>>,
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

fn unsubscribe_from_interval(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe interval_subscription!");
	commands.unsubscribe(example_entities.interval_subscription);
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	interval_subscription: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let observer_entity_commands = commands
		.spawn(Name::new("Observer"))
		.observe(next_number_observer)
		.id();

	let mut interval_observable_entity_commands = commands.spawn((
		Name::new("IntervalObservable"),
		IntervalObservableComponent::new(IntervalObservableOptions {
			duration: Duration::from_secs(1),
			start_on_subscribe: true,
		}),
	));

	let interval_subscription = interval_observable_entity_commands
		.subscribe_to_this_scheduled::<i32, (), Update>(RelativeEntity::Other(
			observer_entity_commands,
		));

	commands.insert_resource(ExampleEntities {
		interval_subscription,
	});
}
