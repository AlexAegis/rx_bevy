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

fn main() -> AppExit {
	App::new()
		.add_plugins((
			DefaultPlugins,
			EguiPlugin {
				enable_multipass_for_primary_context: true,
			},
			WorldInspectorPlugin::new(),
			RxPlugin,
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.register_type::<ExampleEntities>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				toggle_subscription_system::<ExampleEntities, DummyEvent, Never, Update, Virtual>(
					KeyCode::Space,
					|res| res.event_observable,
					|res| res.destination_entity,
				),
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				dummy_event_producer,
				despawn_dummy_event_target.run_if(input_just_pressed(KeyCode::KeyC)),
			),
		)
		.run()
}

#[derive(Resource, Reflect)]
struct ExampleEntities {
	watched_entity: Entity,
	event_observable: Entity,
	destination_entity: Entity,
	subscriptions: HashMap<(Entity, Entity), Entity>,
}

// TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Debug, Clone)]
pub struct DummyEvent {
	pub target: Entity,
	pub count: usize,
}

impl ContainsEntity for DummyEvent {
	fn entity(&self) -> Entity {
		self.target
	}
}

fn despawn_dummy_event_target(mut commands: Commands, example_entities: Res<ExampleEntities>) {
	commands
		.entity(example_entities.watched_entity)
		.try_despawn();
}

fn dummy_event_producer(
	mut commands: Commands,
	example_entities: Res<ExampleEntities>,
	time: Res<Time>,
	mut timer: Local<Timer>,
	mut setup: Local<bool>,
	mut count: Local<usize>,
) {
	if !*setup {
		timer.set_duration(Duration::from_millis(500));
		timer.set_mode(TimerMode::Repeating);
		timer.reset();
		println!("Press Space to Subscribe/Unsubscribe!");
		println!("Press C to Despawn the event producer!");
		*setup = true;
	}

	if commands
		.get_entity(example_entities.watched_entity)
		.is_err()
	{
		return;
	}

	timer.tick(time.delta());

	if timer.just_finished() {
		let dummy_event = DummyEvent {
			count: *count,
			target: example_entities.watched_entity,
		};

		println!(
			"Producer is sending {:?} to {}!",
			dummy_event, example_entities.watched_entity
		);
		// TODO(bevy-0.17): commands.trigger(dummy_event);
		let target = dummy_event.target;
		commands.trigger_targets(dummy_event, target);

		*count += 1;
	}
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
		.spawn(Name::new("ExampleObserver"))
		.observe(print_notification_observer::<DummyEvent, Never, Virtual>)
		.id();

	let watched_entity = commands.spawn(Name::new("They are watching me")).id();

	let event_observable = commands
		.spawn((
			Name::new("EventObservable"),
			EventObservable::<DummyEvent>::new(watched_entity, rx_schedule_update_virtual.handle())
				.into_component(),
		))
		.id();

	commands.insert_resource(ExampleEntities {
		subscriptions: HashMap::new(),
		destination_entity,
		event_observable,
		watched_entity,
	});
}
