use rx_core::prelude::*;
use rx_core_subscriber_exhaust::ExhaustSubscriber;
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

	// TODO: Re-enable, buggy!
	// exhaust_subscriber.complete(); // Should not complete as there is an active subscription!

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

	inner_2.next(2);
	inner_2.next(3);
	inner_2.complete();

	exhaust_subscriber.complete();

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
}
