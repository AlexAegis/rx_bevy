use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[path = "./utilities.rs"]
mod utilities;

use utilities::*;

#[test]
fn it_should_be_possible_to_unsubscribe_a_subscription_entity_with_an_event() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let destination = MockObserver::<usize, Never>::default();
	let notifications = destination.get_notification_collector();

	let destination_entity = app
		.world_mut()
		.spawn_empty()
		.observe(collect_notifications_into::<usize, Never>(
			notifications.clone(),
		))
		.id();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let interval_observable_entity = app
		.world_mut()
		.spawn(
			IntervalObservable::new(
				IntervalObservableOptions {
					duration: Duration::from_millis(500),
					start_on_subscribe: false,
					max_emissions_per_tick: 10,
				},
				scheduler_handle.clone(),
			)
			.into_component(),
		)
		.id();

	let mut destination =
		EntityDestination::<usize, Never>::new(destination_entity, scheduler_handle);
	let tracked_teardown = destination.add_tracked_teardown("interval_destination");

	app.update();

	let subscription_entity = app
		.world_mut()
		.commands()
		.subscribe(interval_observable_entity, destination);

	app.update();

	app.world_mut()
		.resource_mut::<Time<Virtual>>()
		.advance_by(Duration::from_millis(1000));
	app.update();

	notifications.lock().assert_notifications(
		"inveral_observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	app.world_mut()
		.commands()
		.entity(subscription_entity)
		.trigger(SubscriptionNotificationEvent::from_notification(
			SubscriptionNotification::Unsubscribe,
			subscription_entity,
		));

	app.world_mut()
		.resource_mut::<Time<Virtual>>()
		.advance_by(Duration::from_millis(2000));
	app.update();

	notifications
		.lock()
		.assert_nth_notification_is_last("interval_observable", 1);

	tracked_teardown.assert_was_torn_down();
}
