use rx_core::prelude::*;
use rx_core_subscriber_higher_order_exhaust::ExhaustSubscriber;
use rx_core_testing::prelude::*;

#[test]
fn should_only_allow_new_inner_observables_once_the_current_one_finished() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();

	exhaust_subscriber.next(inner_1.clone());
	exhaust_subscriber.next(inner_2.clone()); // Should be ignored, the previous one is still active!

	notification_collector
		.lock()
		.assert_is_empty("exhaust_subscriber");

	inner_1.next(0);
	inner_2.next(99); // Should be ignored!
	inner_1.next(1);

	exhaust_subscriber.next(inner_2.clone()); // Should still be ignored, the previous one is still active!
	inner_2.next(98); // Should be ignored!

	inner_1.complete();

	inner_2.next(97); // Should be ignored, as we haven't made a successful attempt yet to subscribe to it!

	exhaust_subscriber.next(inner_2.clone()); // Now it should subscribe to it!
	exhaust_subscriber.complete();

	inner_2.next(2);
	inner_2.next(3);
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_subscriber",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(exhaust_subscriber.is_closed())
}

#[test]
fn should_immediately_complete_if_there_are_no_active_subscriptions() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	exhaust_subscriber.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_subscriber",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(exhaust_subscriber.is_closed())
}

#[test]
fn should_immediately_error_by_an_inner_error() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();

	exhaust_subscriber.next(inner_1.clone());
	exhaust_subscriber.next(inner_2.clone()); // Should be ignored, the previous one is still active!

	notification_collector
		.lock()
		.assert_is_empty("exhaust_subscriber");

	inner_1.next(0);
	let error = "error";
	inner_1.error(error);

	notification_collector.lock().assert_notifications(
		"exhaust_subscriber",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(exhaust_subscriber.is_closed())
}

#[test]
fn should_immediately_error_by_an_upstream_error() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let error = "error";
	exhaust_subscriber.error(error);

	notification_collector.lock().assert_notifications(
		"exhaust_subscriber",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(exhaust_subscriber.is_closed())
}

#[test]
fn should_also_run_teardowns_on_unsubscribe() {
	let destination = MockObserver::<usize, &'static str>::default();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let tracked_teardown = exhaust_subscriber.add_tracked_teardown("exhaust");

	exhaust_subscriber.unsubscribe();

	tracked_teardown.assert_was_torn_down();

	assert!(exhaust_subscriber.is_closed())
}

#[test]
fn should_also_run_teardowns_immediately_when_added_to_an_already_closed_subscriber() {
	let destination = MockObserver::<usize, &'static str>::default();

	let mut exhaust_subscriber =
		ExhaustSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	exhaust_subscriber.unsubscribe();

	let tracked_teardown = exhaust_subscriber.add_tracked_teardown("exhaust");

	tracked_teardown.assert_was_torn_down();

	assert!(exhaust_subscriber.is_closed())
}
