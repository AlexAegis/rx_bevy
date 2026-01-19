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
use rx_core_common::Never;

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
			RxSchedulerPlugin::<FixedUpdate, Fixed>::default(),
			RxSchedulerPlugin::<Update, Virtual>::default(),
			RxSchedulerPlugin::<Update, Real>::default(),
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
				toggle_subscription_system::<ExampleEntities, usize, Never, FixedUpdate, Fixed>(
					KeyCode::KeyO,
					|res| res.interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Real>(
					KeyCode::KeyP,
					|res| res.interval_observable,
					|res| res.destination_entity_real_clock,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyI,
					|res| res.proxy_interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Real>(
					KeyCode::KeyU,
					|res| res.proxy_interval_observable,
					|res| res.destination_entity_real_clock,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Virtual>(
					KeyCode::KeyL,
					|e| e.keyboard_switch_map_to_interval_observable,
					|res| res.destination_entity,
				),
				toggle_subscription_system::<ExampleEntities, usize, Never, Update, Real>(
					KeyCode::KeyR,
					|e| e.keyboard_switch_map_to_interval_observable,
					|res| res.destination_entity_real_clock,
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
	destination_entity_real_clock: Entity,
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

fn setup(mut commands: Commands, rx_schedule_update_virtual: RxSchedule<Update, Virtual>) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(2., 6., 8.).looking_at(Vec3::ZERO, Vec3::Y),
	));

	let destination_entity = commands
		.spawn(Name::new("Destination (Virtual Clock Logging)"))
		.observe(print_notification_observer::<String, Never, Virtual>)
		.observe(print_notification_observer::<(usize, usize), Never, Virtual>)
		.observe(print_notification_observer::<usize, Never, Virtual>)
		.observe(print_notification_observer::<KeyCode, Never, Virtual>)
		.id();

	let destination_entity_real_clock = commands
		.spawn(Name::new("Destination (Real Clock Logging)"))
		.observe(print_notification_observer::<String, Never, Real>)
		.observe(print_notification_observer::<(usize, usize), Never, Real>)
		.observe(print_notification_observer::<usize, Never, Real>)
		.observe(print_notification_observer::<KeyCode, Never, Real>)
		.id();

	let _virtual_time_setting_subscription = commands
		.with_observable(
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::JustPressed,
				},
				rx_schedule_update_virtual.handle(),
			),
			rx_schedule_update_virtual.handle(),
		)
		.subscribe(ResourceDestination::new(
			|mut virtual_time: Mut<'_, Time<Virtual>>, signal| {
				let speed = match signal {
					ObserverNotification::Next(key_code) => match key_code {
						KeyCode::KeyX => 0.5,
						KeyCode::KeyC => 1.5,
						KeyCode::KeyV => 2.5,
						_ => 1.0,
					},
					_ => 1.0,
				};

				println!("Setting the virtual clocks relative speed to {speed}!");

				virtual_time.set_relative_speed(speed);
			},
			rx_schedule_update_virtual.handle(),
		));

	let instant_subscription_entity = {
		let mut interval_entity = commands.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(
				IntervalObservableOptions {
					duration: Duration::from_millis(200),
					start_on_subscribe: true,
					max_emissions_per_tick: 2,
				},
				rx_schedule_update_virtual.handle(),
			)
			.into_component(),
		));

		let interval_entity_as_observable =
			interval_entity.as_observable::<usize, Never>(rx_schedule_update_virtual.handle());

		let subscription = interval_entity_as_observable
			.enumerate()
			.finalize(|| println!("Finalize from the instant subscription!"))
			.take(10)
			.subscribe(EntityDestination::new(
				destination_entity_real_clock,
				rx_schedule_update_virtual.handle(),
			));
		subscription.entity()
	};

	let keyboard_observable = commands
		.spawn((
			Name::new("KeyboardObservable"),
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::JustPressed,
				},
				rx_schedule_update_virtual.handle(),
			)
			.delay(
				Duration::from_millis(1000),
				rx_schedule_update_virtual.handle(),
			)
			.into_component(),
		))
		.id();

	let interval_observable = commands
		.spawn((
			Name::new("IntervalObservable"),
			IntervalObservable::new(
				IntervalObservableOptions {
					duration: Duration::from_millis(1000),
					start_on_subscribe: true,
					max_emissions_per_tick: 2,
				},
				rx_schedule_update_virtual.handle(),
			)
			.into_component(),
		))
		.id();

	let proxy_interval_observable = commands
		.spawn((
			Name::new(format!(
				"ProxyObservable (IntervalObservable) {}",
				interval_observable
			)),
			ProxyObservable::<usize, Never>::new(
				interval_observable,
				rx_schedule_update_virtual.handle(),
			)
			.into_component(),
		))
		.id();

	let schedule_update_virtual = rx_schedule_update_virtual.handle();
	let keyboard_switch_map_to_interval_observable = commands
		.spawn((
			Name::new("KeyboardSwitchMapToIntervalObservable"),
			KeyboardObservable::new(default(), rx_schedule_update_virtual.handle())
				.filter(|key_code, _| {
					matches!(
						key_code,
						KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4
					)
				})
				.start_with(KeyCode::Digit3)
				.switch_map(
					move |key_code| {
						let duration = match key_code {
							KeyCode::Digit1 => Duration::from_millis(5),
							KeyCode::Digit2 => Duration::from_millis(100),
							KeyCode::Digit3 => Duration::from_millis(500),
							KeyCode::Digit4 => Duration::from_millis(2000),
							_ => unreachable!(),
						};
						println!("Switching to a new inner observable with duration: {duration:?}");
						IntervalObservable::new(
							IntervalObservableOptions {
								duration,
								start_on_subscribe: false,
								max_emissions_per_tick: 4,
							},
							schedule_update_virtual.clone(),
						)
					},
					Never::map_into(),
				)
				.scan(|acc, _next| acc + 1, 0_usize)
				.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		destination_entity,
		destination_entity_real_clock,
		keyboard_observable,
		interval_observable,
		proxy_interval_observable,
		keyboard_switch_map_to_interval_observable,
		instant_subscription_entity,
	});
}
