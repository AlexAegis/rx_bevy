use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
};
use bevy_egui::EguiPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{SubscriptionMapResource, send_message, toggle_subscription_system};
use rx_bevy::prelude::*;
use rx_bevy_context::RxSignal;

/// Press K to start the subscription, then Space to trigger the envelope
fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			//  EguiPlugin::default(), TODO(bevy-0.17): EguiPlugin::default()
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<ExampleEntities, AdsrSignal, Never, Update, Virtual>(
					KeyCode::KeyK,
					|res| res.adsr_observable,
					|res| res.adsr_destination_cube,
				),
				(send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),),
			),
		)
		.run()
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(20., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let adsr_observable = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::WhilePressed,
				},
				rx_schedule_update_virtual.handle(),
			)
			.map(Into::<Option<KeyCode>>::into)
			.fallback_when_silent(
				|_, _, _| Default::default(),
				rx_schedule_update_virtual.handle(),
			) // When nothing pressed, emit the default of the input type
			.map(|key| matches!(key, Some(KeyCode::Space)))
			.map_into::<AdsrTrigger, Never>()
			.adsr(
				AdsrOperatorOptions {
					always_emit_none: false,
					reset_input_on_tick: false,
					envelope: AdsrEnvelope {
						attack_time: Duration::from_millis(250),
						attack_easing: Some(EaseFunction::SineIn),
						decay_time: Duration::from_millis(500),
						decay_easing: Some(EaseFunction::SmoothStep),
						sustain_volume: 0.9,
						release_time: Duration::from_millis(1500),
						release_easing: Some(EaseFunction::CircularOut),
					},
				},
				rx_schedule_update_virtual.handle(),
			)
			.tap_next(|n| println!("tap: {n:?}"))
			.into_component(),
		))
		.id();

	let adsr_destination_cube = commands
		.spawn((
			Name::new("target"),
			Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
			MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
		))
		.observe(handle_move_signal)
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		adsr_observable,
		adsr_destination_cube,
	});
}

fn handle_move_signal(
	next: Trigger<RxSignal<AdsrSignal>>,
	mut transform_query: Query<&mut Transform>,
) {
	if let ObserverNotification::Next(adsr_signal) = next.signal()
		&& let Ok(mut transform) = transform_query.get_mut(next.entity())
	{
		transform.translation += Vec3::X * 0.09 * adsr_signal.value;
	}
}

#[derive(Resource)]
struct ExampleEntities {
	subscriptions: HashMap<(Entity, Entity), Entity>,
	adsr_observable: Entity,
	adsr_destination_cube: Entity,
}

impl SubscriptionMapResource for ExampleEntities {
	fn insert(
		&mut self,
		observable_destination_key: (Entity, Entity),
		subscription_entity: Entity,
	) {
		self.subscriptions
			.insert(observable_destination_key, subscription_entity);
	}

	fn remove(&mut self, observable_destination_key: (Entity, Entity)) -> Option<Entity> {
		self.subscriptions.remove(&observable_destination_key)
	}
}
