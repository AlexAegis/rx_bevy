use rx_core::prelude::*;
use rx_core_common::{ErasedSubscribeObservableExtension, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn erased_subscribe_boxes_destination_and_supports_teardown() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut source = PublishSubject::<usize, MockError>::default();

	let mut subscription = source.erased_subscribe(Box::new(destination));
	let teardown = subscription.add_tracked_teardown("erased_subscribe_teardown");

	source.next(42);
	source.complete();

	notifications.lock().assert_notifications(
		"erased_subscribe_boxes_destination_and_supports_teardown",
		0,
		[
			SubscriberNotification::Next(42),
			SubscriberNotification::Complete,
		],
		true,
	);

	subscription.unsubscribe();
	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}
