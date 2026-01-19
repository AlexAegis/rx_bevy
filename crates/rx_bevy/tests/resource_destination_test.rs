use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[derive(Resource, Default)]
struct MockResource<In, InError>
where
	In: Signal,
	InError: Signal,
{
	notifications: NotificationCollector<In, InError>,
}

#[test]
fn signals_should_reach_the_resource_destination_and_close_on_error() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
	app.init_resource::<MockResource<usize, MockError>>();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let mut destination =
		ResourceDestination::<usize, MockError, MockResource<usize, MockError>, _, _>::new(
			|mock_resource, notification| {
				mock_resource.notifications.lock().push(notification.into())
			},
			scheduler_handle,
		)
		.upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.error(MockError);

	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	app.world()
		.resource::<MockResource<usize, MockError>>()
		.notifications
		.lock()
		.assert_notifications(
			"entity_destination_error",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Error(MockError),
			],
			true,
		);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}

#[test]
fn signals_should_reach_the_resource_destination_and_close_on_complete() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
	app.init_resource::<MockResource<usize, MockError>>();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let mut destination =
		ResourceDestination::<usize, MockError, MockResource<usize, MockError>, _, _>::new(
			|mock_resource, notification| {
				mock_resource.notifications.lock().push(notification.into())
			},
			scheduler_handle,
		)
		.upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.complete();

	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	app.world()
		.resource::<MockResource<usize, MockError>>()
		.notifications
		.lock()
		.assert_notifications(
			"entity_destination_error",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}

#[test]
fn signals_should_reach_the_resource_destination_and_close_on_unsubscribe() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
	app.init_resource::<MockResource<usize, MockError>>();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let mut destination =
		ResourceDestination::<usize, MockError, MockResource<usize, MockError>, _, _>::new(
			|mock_resource, notification| {
				mock_resource.notifications.lock().push(notification.into())
			},
			scheduler_handle,
		)
		.upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.unsubscribe();

	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	app.world()
		.resource::<MockResource<usize, MockError>>()
		.notifications
		.lock()
		.assert_notifications(
			"entity_destination_error",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
			],
			true,
		);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}
