use rx_core::prelude::*;
use rx_core_common::{SubscriptionData, Teardown, TeardownCollection};
use rx_core_testing::{mute_panic, prelude::*};

mod when_some {
	use super::*;

	#[test]
	fn it_should_forward_next_and_complete() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut optional_destination = Some(destination);

		optional_destination.next(1);
		optional_destination.complete();

		notifications.lock().assert_notifications(
			"it_should_forward_next_and_complete",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn it_should_forward_error() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut optional_destination = Some(destination);

		optional_destination.error(MockError);

		notifications.lock().assert_notifications(
			"it_should_forward_error",
			0,
			[SubscriberNotification::Error(MockError)],
			true,
		);
	}

	#[test]
	fn it_should_execute_subscription_teardown_on_unsubscribe() {
		let mut optional_subscription = Some(SubscriptionData::default());
		let (teardown, tracker) =
			Teardown::tracked("it_should_execute_subscription_teardown_on_unsubscribe");

		optional_subscription.add_teardown(teardown);
		tracker.assert_yet_to_be_torn_down();

		optional_subscription.unsubscribe();
		tracker.assert_was_torn_down();
		assert!(optional_subscription.is_closed());
	}
}

mod when_none {
	use super::*;

	#[test]
	fn it_should_ignore_next_and_stay_closed() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut optional_destination: Option<MockObserver<usize, MockError>> = None;
		optional_destination.next(1);
		notifications
			.lock()
			.assert_is_empty("it_should_ignore_next_and_stay_closed");
		assert!(optional_destination.is_closed());
	}

	#[test]
	fn it_should_ignore_complete_and_stay_closed() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut optional_destination: Option<MockObserver<usize, MockError>> = None;
		optional_destination.complete();
		notifications
			.lock()
			.assert_is_empty("it_should_ignore_complete_and_stay_closed");
		assert!(optional_destination.is_closed());
	}

	#[test]
	#[should_panic]
	fn it_should_panic_on_error_but_is_muted() {
		let mut optional_destination: Option<MockObserver<usize, MockError>> = None;
		mute_panic(|| optional_destination.error(MockError));
	}

	#[test]
	fn it_should_treat_unsubscribe_as_no_op_and_stay_closed() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut optional_destination: Option<MockObserver<usize, MockError>> = None;
		optional_destination.unsubscribe();
		notifications
			.lock()
			.assert_is_empty("it_should_treat_unsubscribe_as_no_op_and_stay_closed");
		assert!(optional_destination.is_closed());
	}

	#[test]
	fn it_should_execute_teardown_immediately_for_none_subscription() {
		let mut optional_subscription: Option<SubscriptionData> = None;
		let (teardown, tracker) =
			Teardown::tracked("it_should_execute_teardown_immediately_for_none_subscription");

		optional_subscription.add_teardown(teardown);
		tracker.assert_was_torn_down();
		assert!(optional_subscription.is_closed());
	}
}
