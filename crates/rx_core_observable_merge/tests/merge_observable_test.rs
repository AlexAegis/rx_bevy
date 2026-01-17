use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_from_all_sources_when_any_of_them_nexts() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_1 = PublishSubject::<usize>::default();
	let mut source_2 = PublishSubject::<usize>::default();
	let mut source_3 = PublishSubject::<usize>::default();
	let mut merged = merge(
		(source_1.clone(), source_2.clone(), source_3.clone()),
		usize::MAX,
	);

	let _subscription = merged.subscribe(destination);

	source_1.next(0);
	source_3.next(1);
	source_2.next(2);
	source_1.next(3);
	source_1.next(4);
	source_3.next(5);
	source_1.complete();
	source_3.next(6);
	source_3.complete();
	source_2.next(7);
	source_2.complete();

	notification_collector.lock().assert_notifications(
		"merge",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(6),
			SubscriberNotification::Next(7),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_complete_if_all_inputs_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_1 = PublishSubject::<usize>::default();
	let mut source_2 = PublishSubject::<usize>::default();
	let mut source_3 = PublishSubject::<usize>::default();

	let mut subscription = merge(
		(source_1.clone(), source_2.clone(), source_3.clone()),
		usize::MAX,
	)
	.subscribe(destination);

	assert!(
		notification_collector.lock().is_empty(),
		"no emissions before nexts"
	);

	source_1.next(1);

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Next(1)
	);

	source_2.next(2);

	assert_eq!(
		notification_collector.lock().nth_notification(1),
		&SubscriberNotification::Next(2)
	);

	source_1.complete();
	source_3.complete(); // This source never emitted

	source_2.next(3);
	source_2.complete();

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"merge",
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
fn should_complete_when_inputs_immediately_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_1 = PublishSubject::<usize>::default();
	source_1.complete();
	let mut source_2 = PublishSubject::<usize>::default();
	source_2.complete();

	let mut subscription =
		merge((source_1.clone(), source_2.clone()), usize::MAX).subscribe(destination);

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"merge",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_not_complete_until_all_completes() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_1 = PublishSubject::<usize>::default();
	let mut source_2 = PublishSubject::<usize>::default();
	let mut source_3 = PublishSubject::<usize>::default();
	let mut merged = merge(
		(source_1.clone(), source_2.clone(), source_3.clone()),
		usize::MAX,
	);

	let _subscription = merged.subscribe(destination);

	source_1.complete();
	source_2.complete();

	notification_collector.lock().assert_is_empty("merge");

	source_3.complete();

	notification_collector.lock().assert_notifications(
		"merge",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_error_when_any_errors() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_1 = PublishSubject::<usize, &'static str>::default();
	let mut source_2 = PublishSubject::<usize, &'static str>::default();
	let mut source_3 = PublishSubject::<usize, &'static str>::default();
	let mut merged = merge(
		(source_1.clone(), source_2.clone(), source_3.clone()),
		usize::MAX,
	);

	let _subscription = merged.subscribe(destination);

	let error = "error";
	source_1.complete();
	source_2.error(error);
	source_3.complete();

	notification_collector.lock().assert_notifications(
		"merge",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

mod concurrency_limit {
	use super::*;

	#[test]
	fn should_only_subscribe_to_as_many_input_observables_as_concurrency_limit_allows() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source_1 = PublishSubject::<usize>::default();
		let mut source_2 = PublishSubject::<usize>::default();
		let mut source_3 = PublishSubject::<usize>::default();
		let mut merged = merge((source_1.clone(), source_2.clone(), source_3.clone()), 2);

		let _subscription = merged.subscribe(destination);

		source_1.next(1);
		source_2.next(2);
		source_3.next(3); // Should not be observed

		notification_collector.lock().assert_notifications(
			"merge",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
			],
			true,
		);

		source_1.complete();

		source_3.next(4);
		source_2.next(5);

		notification_collector.lock().assert_notifications(
			"merge",
			2,
			[
				SubscriberNotification::Next(4),
				SubscriberNotification::Next(5),
			],
			true,
		);
	}

	#[test]
	fn should_complete_even_if_the_not_yet_subscribed_source_was_already_completed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source_1 = PublishSubject::<usize>::default();
		let mut source_2 = PublishSubject::<usize>::default();
		let mut source_3 = PublishSubject::<usize>::default();
		let mut merged = merge((source_1.clone(), source_2.clone(), source_3.clone()), 2);

		let _subscription = merged.subscribe(destination);

		source_1.next(1);
		source_2.next(2);
		source_3.complete(); // Pre-complete, it is not yet observed

		notification_collector.lock().assert_notifications(
			"merge",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
			],
			true,
		);

		source_1.complete(); // Subscribes to source_3, but it's already completed
		source_2.complete();

		notification_collector.lock().assert_notifications(
			"merge",
			2,
			[SubscriberNotification::Complete],
			true,
		);
	}

	#[test]
	fn should_treat_concurrency_limit_0_as_1() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source_1 = PublishSubject::<usize>::default();
		let mut source_2 = PublishSubject::<usize>::default();
		let mut source_3 = PublishSubject::<usize>::default();
		let mut merged = merge((source_1.clone(), source_2.clone(), source_3.clone()), 0);

		let _subscription = merged.subscribe(destination);

		source_1.next(1);
		source_2.next(2); // Concurrency limit is 1, so this isn't observed
		source_3.next(3);

		notification_collector.lock().assert_notifications(
			"merge",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		source_1.complete(); // Subscribes to source_3, but it's already completed
		source_2.complete();
		source_3.next(4);
		source_3.complete();

		notification_collector.lock().assert_notifications(
			"merge",
			1,
			[
				SubscriberNotification::Next(4),
				SubscriberNotification::Complete,
			],
			true,
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("source_1");

		let source_2 = PublishSubject::<usize, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("source_2");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"merge",
			merge(
				(
					source_1
						.clone()
						.finalize(move || source_1_finalized.unsubscribe()),
					source_2
						.clone()
						.finalize(move || source_2_finalized.unsubscribe()),
				),
				usize::MAX,
			),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		source_1.error(MockError);

		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("source_1");

		let mut source_2 = PublishSubject::<usize, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("source_2");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"merge",
			merge(
				(
					source_1
						.clone()
						.finalize(move || source_1_finalized.unsubscribe()),
					source_2
						.clone()
						.finalize(move || source_2_finalized.unsubscribe()),
				),
				usize::MAX,
			),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		source_1.complete();
		source_2.complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("source_1");

		let source_2 = PublishSubject::<usize, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("source_2");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"merge",
			merge(
				(
					source_1
						.clone()
						.finalize(move || source_1_finalized.unsubscribe()),
					source_2
						.clone()
						.finalize(move || source_2_finalized.unsubscribe()),
				),
				usize::MAX,
			),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("source_1");

		let mut source_2 = PublishSubject::<usize, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("source_2");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"merge",
			merge(
				(
					source_1
						.clone()
						.finalize(move || source_1_finalized.unsubscribe()),
					source_2
						.clone()
						.finalize(move || source_2_finalized.unsubscribe()),
				),
				usize::MAX,
			),
		);
		let observable = harness.create_harness_observable().take(2);
		harness.subscribe_to(observable);

		source_1.next(1);
		source_2.next(2);

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let source_1 = PublishSubject::<usize, MockError>::default();
		let source_2 = PublishSubject::<usize, MockError>::default();

		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("source_1");

		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("source_2");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"merge",
			merge(
				(
					source_1
						.finalize(move || source_1_finalized.unsubscribe())
						.clone(),
					source_2
						.finalize(move || source_2_finalized.unsubscribe())
						.clone(),
				),
				usize::MAX,
			),
		);
		let observable = harness.create_harness_observable().take(0);
		harness.subscribe_to(observable);

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}
}
