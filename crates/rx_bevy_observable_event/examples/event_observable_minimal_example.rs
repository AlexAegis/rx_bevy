use std::time::Duration;

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
			RxSchedulerPlugin::<Update, Virtual>::default(),
		))
		.init_resource::<ExampleSubscriptions>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				send_message(AppExit::Success).run_if(input_just_pressed(KeyCode::Escape)),
				dummy_event_producer,
			),
		)
		.run()
}

// TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Debug, Clone)]
pub struct DummyEvent {
	pub target: Entity,
}

impl ContainsEntity for DummyEvent {
	fn entity(&self) -> Entity {
		self.target
	}
}

fn dummy_event_producer(
	mut commands: Commands,
	dummy_event_target: Res<DummyEventTarget>,
	time: Res<Time>,
	mut timer: Local<Timer>,
	mut setup: Local<bool>,
) {
	if !*setup {
		timer.set_duration(Duration::from_millis(500));
		timer.set_mode(TimerMode::Repeating);
		timer.reset();
		*setup = true;
	}

	timer.tick(time.delta());

	if timer.just_finished() {
		let dummy_event = DummyEvent {
			target: **dummy_event_target,
		};

		println!(
			"Producer is sending {:?} to {}!",
			dummy_event, **dummy_event_target
		);

		let target = dummy_event.target;
		commands.trigger_targets(dummy_event, target);
	}
}

#[derive(Resource, Deref, DerefMut)]
pub struct DummyEventTarget(Entity);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ExampleSubscriptions(SharedSubscription);

fn setup(
	mut commands: Commands,
	rx_schedule_update_virtual: RxSchedule<Update, Virtual>,
	mut subscriptions: ResMut<ExampleSubscriptions>,
) {
	let watched_entity = commands.spawn(Name::new("Watch me")).id();

	subscriptions.add(
		EventObservable::<DummyEvent>::new(watched_entity, rx_schedule_update_virtual.handle())
			.subscribe(PrintObserver::new("event_observable")),
	);

	commands.insert_resource(DummyEventTarget(watched_entity));
}
