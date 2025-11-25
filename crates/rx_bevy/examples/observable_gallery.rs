use std::time::Duration;

use bevy::{
	input::common_conditions::input_just_pressed, platform::collections::HashMap, prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use examples_common::{
	SubscriptionMapResource, print_notification_observer, send_message, toggle_subscription_system,
};
use rx_bevy::prelude::*;
use rx_core_traits::Never;

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
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<ExampleEntities, KeyCode, Never, Update, Virtual>(
					KeyCode::KeyK,
					|res| res.keyboard_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyO,
					|res| res.interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyP,
					|res| res.interval_observable,
					|res| res.destination_entity_2,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyI,
					|res| res.proxy_interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyU,
					|res| res.proxy_interval_observable,
					|res| res.destination_entity_2,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyL,
					|e| e.keyboard_switch_map_to_interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, usize, Update, Virtual>(
					KeyCode::KeyM, // This will (intentionally) miss as the Output types don't match with an observable!
					|res| res.interval_observable,
					|res| res.destination_entity,
				),
				despawn_instant_subscription.run_if(input_just_pressed(KeyCode::KeyQ)),
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
			),
		)
		.run()
}

fn despawn_instant_subscription(mut commands: Commands, r: Res<ExampleEntities>) {
	commands.entity(r.instant_subscription_entity).despawn();
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	destination_entity: Entity,
	destination_entity_2: Entity,
	subscriptions: HashMap<(Entity, Entity), Entity>,
	keyboard_observable: Entity,
	keyboard_switch_map_to_interval_observable: Entity,
	interval_observable: Entity,
	proxy_interval_observable: Entity,
	instant_subscription_entity: Entity,
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

fn setup(mut commands: Commands, mut context: RxBevyContextItem) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let destination_entity = commands
		.spawn(Name::new("Destination"))
		.observe(print_notification_observer::<String, Never>)
		.observe(print_notification_observer::<(usize, usize), Never>)
		.observe(print_notification_observer::<usize, Never>)
		.observe(print_notification_observer::<KeyCode, Never>)
		.id();

	let destination_entity_2 = commands
		.spawn(Name::new("Destination 2"))
		.observe(print_notification_observer::<String, Never>)
		.observe(print_notification_observer::<(usize, usize), Never>)
		.observe(print_notification_observer::<usize, Never>)
		.observe(print_notification_observer::<KeyCode, Never>)
		.id();

	let instant_subscription_entity = {
		let mut interval_entity = commands.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(IntervalObservableOptions {
				duration: Duration::from_millis(200),
				start_on_subscribe: true,
				max_emissions_per_tick: 2,
			})
			.into_component(),
		));

		let interval_entity_as_observable =
			interval_entity.as_observable::<usize, Never, Update, Virtual>();

		let subscription = interval_entity_as_observable
			.enumerate()
			.finalize(|_| println!("Finalize from the instant subscription!"))
			.take(10)
			.subscribe(EntityDestination::new(destination_entity_2), &mut context);

		subscription.into_entity()
	};

	let keyboard_observable = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::new(KeyboardObservableOptions {
				emit: KeyboardObservableEmit::JustPressed,
			})
			.into_component(),
		))
		.id();

	let interval_observable = commands
		.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(IntervalObservableOptions {
				duration: Duration::from_millis(1000),
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
			ProxyObservable::<usize, Never, Update, Virtual>::new(interval_observable)
				.into_component(),
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
				})
				.scan(|acc, _next| acc + 1, 0_usize)
				.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		destination_entity,
		destination_entity_2,
		keyboard_observable,
		interval_observable,
		proxy_interval_observable,
		keyboard_switch_map_to_interval_observable,
		instant_subscription_entity,
	});
}
