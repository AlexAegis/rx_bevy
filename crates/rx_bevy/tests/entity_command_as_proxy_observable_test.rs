use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn entity_commands_can_expose_proxy_observable() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let source_entity = app
		.world_mut()
		.commands()
		.spawn((1..=3_usize).into_observable().into_component())
		.id();

	let destination = MockObserver::<usize, Never>::default();
	let notification_collector = destination.get_notification_collector();

	let mut subscription = app
		.world_mut()
		.commands()
		.entity(source_entity)
		.as_proxy_observable::<usize, Never>(scheduler_handle)
		.subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("entity_command_as_proxy_observable");

	app.update();

	notification_collector.lock().assert_notifications(
		"entity_command_as_proxy_observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);

	tracked_teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}
