use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_complete_if_all_inputs_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<usize>::default();
	let mut subject_3 = PublishSubject::<usize>::default();

	let mut subscription =
		ConcatObservable::new((subject_1.clone(), subject_2.clone(), subject_3.clone()))
			.subscribe(destination);

	assert!(
		notification_collector.lock().is_empty(),
		"nothing should happen when subscribed to non replaying sources"
	);

	subject_1.next(1);

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Next(1)
	);

	subject_2.next(2);

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"should not be subscribed to the second source until the first one completes"
	);

	subject_1.complete();
	subject_3.complete(); // This will never emit

	subject_2.next(2);
	subject_2.next(3);
	subject_2.complete();

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concat",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_immediately_complete_all_inputs_immediately_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	subject_1.complete();
	let mut subject_2 = PublishSubject::<usize>::default();
	subject_2.complete();

	let mut subscription = concat((subject_1.clone(), subject_2.clone())).subscribe(destination);

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concat",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut subject_1 = PublishSubject::<usize, TestError>::default();
		let mut subject_1_finalize = SharedSubscription::default();
		let subject_1_tracked_teardown = subject_1_finalize.add_tracked_teardown("subject_1");

		let subject_2 = PublishSubject::<usize, TestError>::default();
		let mut subject_2_finalize = SharedSubscription::default();
		let subject_2_tracked_teardown = subject_2_finalize.add_tracked_teardown("subject_2");

		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"concat",
			concat((
				subject_1
					.clone()
					.finalize(move || subject_1_finalize.unsubscribe()),
				subject_2
					.clone()
					.finalize(move || subject_2_finalize.unsubscribe()),
			)),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		subject_1.error(TestError);

		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));

		subject_1_tracked_teardown.assert_was_torn_down();
		subject_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut subject_1 = PublishSubject::<usize, TestError>::default();
		let mut subject_1_finalize = SharedSubscription::default();
		let subject_1_tracked_teardown = subject_1_finalize.add_tracked_teardown("subject_1");

		let mut subject_2 = PublishSubject::<usize, TestError>::default();
		let mut subject_2_finalize = SharedSubscription::default();
		let subject_2_tracked_teardown = subject_2_finalize.add_tracked_teardown("subject_2");

		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"concat",
			concat((
				subject_1
					.clone()
					.finalize(move || subject_1_finalize.unsubscribe()),
				subject_2
					.clone()
					.finalize(move || subject_2_finalize.unsubscribe()),
			)),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		subject_1.next(1);
		subject_1.complete();
		subject_2.next(2);
		subject_2.complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		harness.notifications().assert_notifications(
			"concat",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		subject_1_tracked_teardown.assert_was_torn_down();
		subject_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let subject_1 = PublishSubject::<usize, TestError>::default();
		let mut subject_1_finalize = SharedSubscription::default();
		let subject_1_tracked_teardown = subject_1_finalize.add_tracked_teardown("subject_1");

		let subject_2 = PublishSubject::<usize, TestError>::default();

		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"concat",
			concat((
				subject_1
					.clone()
					.finalize(move || subject_1_finalize.unsubscribe()),
				subject_2.clone(), // Not even subscribing to this one
			)),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		subject_1_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut subject_1 = PublishSubject::<usize, TestError>::default();
		let mut subject_1_finalize = SharedSubscription::default();
		let subject_1_tracked_teardown = subject_1_finalize.add_tracked_teardown("subject_1");

		let subject_2 = PublishSubject::<usize, TestError>::default();
		let mut subject_2_finalize = SharedSubscription::default();
		let subject_2_tracked_teardown = subject_2_finalize.add_tracked_teardown("subject_2");

		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"concat",
			concat((
				subject_1
					.clone()
					.finalize(move || subject_1_finalize.unsubscribe()),
				subject_2
					.clone()
					.finalize(move || subject_2_finalize.unsubscribe()),
			)),
		);
		let observable = harness.create_harness_observable().take(2);
		harness.subscribe_to(observable);

		subject_1.next(1);
		subject_1.next(2);
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		subject_1_tracked_teardown.assert_was_torn_down();
		subject_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let subject_1 = PublishSubject::<usize, TestError>::default();
		let mut subject_1_finalize = SharedSubscription::default();
		let subject_1_tracked_teardown = subject_1_finalize.add_tracked_teardown("subject_1");

		let subject_2 = PublishSubject::<usize, TestError>::default();
		let mut subject_2_finalize = SharedSubscription::default();
		let subject_2_tracked_teardown = subject_2_finalize.add_tracked_teardown("subject_2");

		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"concat",
			concat((
				subject_1
					.clone()
					.finalize(move || subject_1_finalize.unsubscribe()),
				subject_2
					.clone()
					.finalize(move || subject_2_finalize.unsubscribe()),
			)),
		);
		let observable = harness.create_harness_observable().take(0);
		harness.subscribe_to(observable);

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		subject_1_tracked_teardown.assert_was_torn_down();
		subject_2_tracked_teardown.assert_was_torn_down();
	}
}
