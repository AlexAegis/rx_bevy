use rx_core::prelude::*;
use rx_core_common::EraseSubscriptionExtension;
use rx_core_testing::prelude::*;

#[test]
fn erased_subscription_unsubscribes_and_executes_teardown() {
	let destination = MockObserver::<usize, MockError>::default();
	let notification_collector = destination.get_notification_collector();
	let mut source = PublishSubject::<usize, MockError>::default();

	let mut subscription = source.clone().subscribe(destination).erase();
	let teardown = subscription.add_tracked_teardown("erased_subscription_unsubscribes");

	subscription.unsubscribe();

	assert!(subscription.is_closed());
	teardown.assert_was_torn_down();
	notification_collector.lock().assert_notifications(
		"erased_subscription_unsubscribes_and_executes_teardown",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	source.next(1);
	notification_collector.lock().assert_notifications(
		"erased_subscription_unsubscribes_and_executes_teardown",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn erased_subscription_executes_teardown_on_drop() {
	let destination = MockObserver::<usize, MockError>::default();
	let notification_collector = destination.get_notification_collector();
	let mut source = PublishSubject::<usize, MockError>::default();

	let teardown = {
		let mut subscription = source.subscribe(destination).erase();
		subscription.add_tracked_teardown("erased_subscription_drop")
	};

	teardown.assert_was_torn_down();
	notification_collector.lock().assert_notifications(
		"erased_subscription_executes_teardown_on_drop",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	source.next(1);
	notification_collector.lock().assert_notifications(
		"erased_subscription_executes_teardown_on_drop",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}
