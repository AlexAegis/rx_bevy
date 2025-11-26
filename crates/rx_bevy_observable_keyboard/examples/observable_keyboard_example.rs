use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::send_message;
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
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				unsubscribe.run_if(input_just_pressed(KeyCode::Space)),
			),
		)
		.run()
}

fn next_number_observer(
	next: Trigger<RxSignal<String>>,
	name_query: Query<&Name>,
	time: Res<Time>,
) {
	println!(
		"value: {:?}\tby {:?}\tname: {:?}\telapsed: {}",
		next.signal(),
		next.entity(),
		name_query.get(next.entity()).unwrap(),
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
				.map(|key_code| format!("KEYCODE {:?}", key_code))
				.into_component(),
		))
		.id();

	let keyboard_event_observer = commands
		.spawn((Name::new("KeyboardObserver"),))
		.observe(next_number_observer)
		.id();

	let subscription = commands.subscribe::<_, Update, Virtual>(
		keyboard_observable_entity,
		EntityDestination::<String, Never>::new(keyboard_event_observer),
	);

	commands.insert_resource(ExampleEntities {
		subscription,
		keyboard_event_observer,
		keyboard_observable_entity,
	});
}
