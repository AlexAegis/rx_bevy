use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_cause_an_error_on_subscribe() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let _subscription = throw(error).subscribe(destination);

	notification_collector.lock().assert_notifications(
		"throw",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}
