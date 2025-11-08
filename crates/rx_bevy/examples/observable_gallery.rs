use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{
	SubscriptionMapResource, print_notification_observer, send_event, toggle_subscription_system,
};
use rx_bevy::prelude::*;
use rx_bevy_context::observable::ProxyObservable;
use rx_core_traits::Never;

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
				toggle_subscription_system::<ExampleEntities, KeyCode, Never>(
					KeyCode::KeyK,
					|res| res.keyboard_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never>(
					KeyCode::KeyI,
					|res| res.proxy_interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never>(
					KeyCode::KeyL,
					|e| e.keyboard_switch_map_to_interval_observable,
					|res| res.destination_entity,
				),
				send_event(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	destination_entity: Entity,
	subscriptions: HashMap<(Entity, Entity), Entity>,
	keyboard_observable: Entity,
	keyboard_switch_map_to_interval_observable: Entity,
	interval_observable: Entity,
	proxy_interval_observable: Entity,
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

fn setup(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let destination_entity = commands
		.spawn(Name::new("Destination"))
		.observe(print_notification_observer::<String, Never>)
		.observe(print_notification_observer::<i32, Never>)
		.observe(print_notification_observer::<usize, Never>)
		.observe(print_notification_observer::<KeyCode, Never>)
		.id();

	let keyboard_observable = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::default().into_component(),
		))
		.id();

	let interval_observable = commands
		.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(IntervalObservableOptions {
				duration: Duration::from_millis(500),
				start_on_subscribe: true,
				max_emissions_per_tick: 2,
			})
			.into_component(),
		))
		.id();

	let proxy_interval_observable = commands
		.spawn((
			Name::new(format!(
				"ProxyObservable (IntervalObservable) {}",
				interval_observable
			)),
			ProxyObservable::<usize, Never>::new(interval_observable).into_component(),
		))
		.id();

	let keyboard_switch_map_to_interval_observable = commands
		.spawn((
			Name::new("KeyboardSwitchMapToIntervalObservable"),
			KeyboardObservable::default()
				.filter(|key_code| {
					matches!(
						key_code,
						KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4
					)
				})
				.switch_map(|key_code| {
					let duration = match key_code {
						KeyCode::Digit1 => Duration::from_millis(5),
						KeyCode::Digit2 => Duration::from_millis(100),
						KeyCode::Digit3 => Duration::from_millis(500),
						KeyCode::Digit4 => Duration::from_millis(2000),
						_ => unreachable!(),
					};
					println!("Switching to a new inner observable with duration: {duration:?}");
					IntervalObservable::new(IntervalObservableOptions {
						duration,
						start_on_subscribe: false,
						max_emissions_per_tick: 4,
					})
					.tap_next(|n, _| println!("inner next {n}"))
				})
				.scan(|acc, _next| acc + 1, 0 as usize)
				.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		destination_entity,
		keyboard_observable,
		interval_observable,
		proxy_interval_observable,
		keyboard_switch_map_to_interval_observable,
	});
}
