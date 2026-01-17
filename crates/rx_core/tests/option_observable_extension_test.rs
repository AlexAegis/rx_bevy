use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[test]
fn option_observable_some_forwards_notifications() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();

	let mut maybe_source = Some(PublishSubject::<usize, MockError>::default());
	let mut subscription = maybe_source.subscribe(destination);
	let teardown = subscription.add_tracked_teardown("option_observable_some");

	maybe_source.as_mut().unwrap().next(1);
	maybe_source.as_mut().unwrap().next(2);
	maybe_source.as_mut().unwrap().complete();

	notifications.lock().assert_notifications(
		"option_observable_some",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn option_observable_none_closes_and_executes_teardown_immediately() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();

	let mut maybe_source: Option<PublishSubject<usize, MockError>> = None;
	let mut subscription = maybe_source.subscribe(destination);
	let teardown = subscription.add_tracked_teardown("option_observable_none");

	assert!(subscription.is_closed());
	teardown.assert_was_torn_down();
	notifications.lock().assert_notifications(
		"option_observable_none_closes_and_executes_teardown_immediately",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}
