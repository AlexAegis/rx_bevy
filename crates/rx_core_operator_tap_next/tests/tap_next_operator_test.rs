use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

mod tap_next {
	use super::*;

	#[test]
	fn forwards_next_notifications_to_the_tap_fn_too() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
		let tap_notification_collector_clone = tap_notification_collector.clone();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let subscription = source
			.clone()
			.tap_next(move |next| {
				tap_notification_collector_clone
					.lock()
					.push(SubscriberNotification::Next(*next))
			})
			.subscribe(destination);

		source.next(0);
		source.next(1);
		assert!(!subscription.is_closed());
		source.complete();

		notification_collector.lock().assert_notifications(
			"tap_next",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);

		tap_notification_collector.lock().assert_notifications(
			"tap_destination",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Next(1),
			],
			true,
		);

		assert!(subscription.is_closed());
	}
}

mod contracts_harness_v2 {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let tap_notification_collector = NotificationCollector::<usize>::default();
		let tap_notification_collector_clone = tap_notification_collector.clone();

		let mut harness = TestHarness::new_operator_harness("tap_next");
		let observable = harness.create_harness_observable().tap_next(move |next| {
			tap_notification_collector_clone
				.lock()
				.push(SubscriberNotification::Next(*next))
		});
		harness.subscribe_to(observable);

		harness.assert_rx_contract_closed_after_error(TestError, TestError);

		tap_notification_collector
			.lock()
			.assert_is_empty("tap_destination - should not have observed anything");
	}
}

mod contracts_harness_v1 {

	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let tap_notification_collector = NotificationCollector::<usize>::default();

		let mut harness = OperatorTestHarness::<_, usize, TestError>::new("tap_next", |upstream| {
			let tap_notification_collector_clone = tap_notification_collector.clone();

			upstream.tap_next(move |next| {
				tap_notification_collector_clone
					.lock()
					.push(SubscriberNotification::Next(*next))
			})
		});

		harness.assert_rx_contract_closed_after_error(TestError, TestError);

		tap_notification_collector
			.lock()
			.assert_is_empty("tap_destination - should not have observed anything");
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let tap_notification_collector = NotificationCollector::<usize>::default();

		let mut harness = OperatorTestHarness::<_, usize, TestError>::new("tap_next", |upstream| {
			let tap_notification_collector_clone = tap_notification_collector.clone();

			upstream.tap_next(move |next| {
				tap_notification_collector_clone
					.lock()
					.push(SubscriberNotification::Next(*next))
			})
		});

		harness.assert_rx_contract_closed_after_complete();

		tap_notification_collector
			.lock()
			.assert_is_empty("tap_destination - should not have observed anything");
	}

	#[test]
	fn assert_rx_contract_closed_after_unsubscribe() {
		let tap_notification_collector = NotificationCollector::<usize>::default();

		let mut harness = OperatorTestHarness::<_, usize, TestError>::new("tap_next", |upstream| {
			let tap_notification_collector_clone = tap_notification_collector.clone();

			upstream.tap_next(move |next| {
				tap_notification_collector_clone
					.lock()
					.push(SubscriberNotification::Next(*next))
			})
		});

		harness.assert_rx_contract_closed_after_unsubscribe();

		tap_notification_collector
			.lock()
			.assert_is_empty("tap_destination - should not have observed anything");
	}
}

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
		let tap_notification_collector_clone = tap_notification_collector.clone();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let composed = compose_operator::<usize, &'static str>().tap_next(move |next| {
			tap_notification_collector_clone
				.lock()
				.push(SubscriberNotification::Next(*next))
		});

		let subscription = source.clone().pipe(composed).subscribe(destination);

		source.complete();

		notification_collector.lock().assert_notifications(
			"tap_next",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		tap_notification_collector
			.lock()
			.assert_is_empty("tap_destination");

		assert!(subscription.is_closed());
	}
}
