use std::time::Duration;

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
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
			RxPlugin,
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
	mut next: Trigger<RxSignal<String>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) {
	println!(
		"value observed: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.event_mut().consume(),
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
	keyboard_observable_entity: Entity,
	keyboard_event_observer: Entity,
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
			KeyboardObservable::default()
				.filter(|key_code| {
					matches!(
						key_code,
						KeyCode::KeyW | KeyCode::KeyA | KeyCode::KeyS | KeyCode::KeyD
					)
				})
				.switch_map(|key_code| {
					// TODO: SwitchMap is unresponsive!!!
					let duration = match key_code {
						KeyCode::KeyW => Duration::from_millis(5),
						KeyCode::KeyA => Duration::from_millis(100),
						KeyCode::KeyS => Duration::from_millis(500),
						KeyCode::KeyD => Duration::from_millis(2000),
						_ => Duration::from_millis(500),
					};
					IntervalObservable::new(IntervalObservableOptions {
						duration,
						start_on_subscribe: true,
						max_emissions_per_tick: 4,
					})
				})
				.map(|key_code| format!("Ticking! {:?}", key_code))
				.into_component(),
		))
		.id();

	let keyboard_event_observer = commands
		.spawn((Name::new("KeyboardObserver"),))
		.observe(next_number_observer)
		.id();

	let subscription = commands
		.subscribe::<String, (), Update>(keyboard_observable_entity, keyboard_event_observer);

	commands.insert_resource(ExampleEntities {
		subscription,
		keyboard_event_observer,
		keyboard_observable_entity,
	});
}
