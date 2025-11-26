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
			RxScheduler::<Update, Virtual>::default(),
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

fn unsubscribe(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	println!("Unsubscribe subscription!");
	commands.entity(example_entities.subscription).despawn();
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	proxy_keyboard_observable_entity: Entity,
	subscription: Entity,
}

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let destination_entity = commands
		.spawn((Name::new("Destination"),))
		.observe(|next: Trigger<RxSignal<String, Never>>| {
			println!("{:?}", next.signal());
		})
		.id();

	let keyboard_observable_entity = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::default().into_component(),
		))
		.id();

	let proxy_keyboard_observable_entity = commands
		.spawn((
			Name::new("Proxy"),
			ProxyObservable::<KeyCode, Never, Update, Virtual>::new(keyboard_observable_entity)
				.map(|key_code| format!("KEYCODE {:?}", key_code))
				.into_component(),
		))
		.id();

	let subscription = commands.subscribe::<_, Update, Virtual>(
		proxy_keyboard_observable_entity,
		EntityDestination::<String, Never>::new(destination_entity),
	);

	commands.insert_resource(ExampleEntities {
		subscription,
		proxy_keyboard_observable_entity,
	});
}
