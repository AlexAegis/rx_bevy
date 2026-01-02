use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_automatically_connect_on_subscribe_and_share_emissions_across_all_subscribers() {
	let destination_1 = MockObserver::<usize, &'static str>::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::<usize, &'static str>::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut shared = source
		.clone()
		.share(ConnectableOptions::<PublishSubject<usize, &'static str>>::default());

	let mut subscription_1 = shared.subscribe(destination_1);
	let teardown_tracker_1 = subscription_1.add_tracked_teardown("share - destination_1");
	let mut subscription_2 = shared.subscribe(destination_2);
	let teardown_tracker_2 = subscription_2.add_tracked_teardown("share - destination_2");

	source.next(0);
	source.next(1);

	notification_collector_1.lock().assert_notifications(
		"share - destination_1",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"share - destination_2",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	subscription_1.unsubscribe();
	subscription_2.unsubscribe();

	assert!(subscription_1.is_closed());
	assert!(subscription_2.is_closed());

	teardown_tracker_1.assert_was_torn_down();
	teardown_tracker_2.assert_was_torn_down();

	assert!(shared.is_connected(), "should stay connected");

	shared.disconnect();

	assert!(!shared.is_connected(), "should disconnect when requested");
}

#[test]
fn should_disconnect_when_the_ref_count_is_zero() {
	let destination_1 = MockObserver::<usize, &'static str>::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::<usize, &'static str>::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut shared = source.clone().share(
		ConnectableOptions::<ReplaySubject<1, _, _>>::default().disconnect_when_ref_count_zero(),
	);

	let mut subscription_1 = shared.subscribe(destination_1);
	let teardown_tracker_1 = subscription_1.add_tracked_teardown("share - destination_1");
	let mut subscription_2 = shared.subscribe(destination_2);
	let teardown_tracker_2 = subscription_2.add_tracked_teardown("share - destination_2");

	assert!(shared.is_connected());

	source.next(0);
	source.next(1);

	notification_collector_1.lock().assert_notifications(
		"share - destination_1",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"share - destination_2",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	subscription_1.unsubscribe();
	subscription_2.unsubscribe();

	assert!(subscription_1.is_closed());
	assert!(subscription_2.is_closed());

	teardown_tracker_1.assert_was_torn_down();
	teardown_tracker_2.assert_was_torn_down();

	assert!(!shared.is_connected(), "should not stay connected");
}

#[test]
fn should_close_when_errored() {
	let destination_1 = MockObserver::<usize, &'static str>::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::<usize, &'static str>::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut shared = source
		.clone()
		.share(ConnectableOptions::<PublishSubject<usize, &'static str>>::default());

	let mut subscription_1 = shared.subscribe(destination_1);
	let teardown_tracker_1 = subscription_1.add_tracked_teardown("share - destination_1");
	let mut subscription_2 = shared.subscribe(destination_2);
	let teardown_tracker_2 = subscription_2.add_tracked_teardown("share - destination_2");

	let error = "error";
	source.error(error);

	notification_collector_1.lock().assert_notifications(
		"share - destination_1",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"share - destination_2",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription_1.is_closed());
	assert!(subscription_2.is_closed());

	teardown_tracker_1.assert_was_torn_down();
	teardown_tracker_2.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination_1 = MockObserver::<usize, &'static str>::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::<usize, &'static str>::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut shared = source
		.clone()
		.share(ConnectableOptions::<PublishSubject<usize, &'static str>>::default());

	let mut subscription_1 = shared.subscribe(destination_1);
	let teardown_tracker_1 = subscription_1.add_tracked_teardown("share - destination_1");
	let mut subscription_2 = shared.subscribe(destination_2);
	let teardown_tracker_2 = subscription_2.add_tracked_teardown("share - destination_2");

	source.complete();

	notification_collector_1.lock().assert_notifications(
		"share - destination_1",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"share - destination_2",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription_1.is_closed());
	assert!(subscription_2.is_closed());

	teardown_tracker_1.assert_was_torn_down();
	teardown_tracker_2.assert_was_torn_down();
}

#[test]
fn should_handle_manual_connections_and_disconnections_and_close_when_completed() {
	let destination_1 = MockObserver::<usize, &'static str>::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::<usize, &'static str>::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut shared = source
		.clone()
		.share(ConnectableOptions::<PublishSubject<usize, &'static str>>::default());

	let mut subscription_1 = shared.subscribe(destination_1);
	let teardown_tracker_1 = subscription_1.add_tracked_teardown("share - destination_1");
	let mut subscription_2 = shared.subscribe(destination_2);
	let teardown_tracker_2 = subscription_2.add_tracked_teardown("share - destination_2");

	assert!(shared.is_connected());
	assert!(
		shared.disconnect(),
		"Did not successfully perform disconnect"
	);
	assert!(!shared.is_connected(), "Did not get disconnected!");
	assert!(
		!shared.disconnect(),
		"Failed to disconnect because it was already disconnected"
	);

	source.next(99);
	notification_collector_1
		.lock()
		.assert_is_empty("share - destination 1");

	notification_collector_2
		.lock()
		.assert_is_empty("share - destination 2");

	assert!(!shared.connect().is_closed(), "Did not connect!");
	assert!(shared.is_connected(), "Did not get connected!");

	source.complete();

	notification_collector_1.lock().assert_notifications(
		"share - destination_1",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"share - destination_2",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription_1.is_closed());
	assert!(subscription_2.is_closed());

	teardown_tracker_1.assert_was_torn_down();
	teardown_tracker_2.assert_was_torn_down();
}
