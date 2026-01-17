use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_only_emit_after_both_observables_emitted_even_if_its_not_in_order() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<&'static str>::default();

	let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

	assert!(
		notification_collector_1.lock().is_empty(),
		"Nothing should've been emitted yet"
	);

	subject_2.next("hello");

	assert!(
		notification_collector_1.lock().is_empty(),
		"Nothing should've been emitted yet after just one of the source emitted"
	);

	subject_1.next(1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next((1, "hello")),
		"the first emission isn't correct"
	);
}

mod before_primed {
	use super::*;

	#[test]
	fn it_should_complete_even_when_only_one_of_the_observables_complete() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Complete,
			"did not complete"
		);
	}

	#[test]
	fn should_unsubscribe_but_not_complete_when_one_of_the_observables_unsubscribe() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Unsubscribe,
			"did not unsubscribe"
		);
	}
}

mod after_primed {
	use super::*;

	#[test]
	fn it_should_complete_even_if_just_one_of_the_observables_complete() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_2.next("hello");
		subject_1.next(1);

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "hello")),
			"the first emission isn't correct"
		);

		subject_1.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Complete,
			"the second emission isn't a complete"
		);
	}

	#[test]
	fn should_unsubscribe_but_not_complete_when_one_of_the_observables_unsubscribe() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_2.next("hello");

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "hello")),
			"did not prime"
		);

		subject_2.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"did not unsubscribe"
		);
	}

	#[test]
	fn should_properly_complete_after_draining_emitting_two_equal_sized_queues_where_the_second_queue_only_starts_after_the_first_finishes()
	 {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_1.next(2);
		subject_1.next(3);
		subject_1.complete();

		assert!(
			!notification_collector_1.lock().nth_notification_exists(0),
			"should not have emitted anything yet"
		);

		subject_2.next("foo");

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "foo")),
			"did not emit the first emission"
		);

		subject_2.next("bar");

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Next((2, "bar")),
			"did not emit the second emission"
		);

		subject_2.next("zed");

		assert_eq!(
			notification_collector_1.lock().nth_notification(2),
			&SubscriberNotification::Next((3, "zed")),
			"did not emit the second emission"
		);

		// Even without the second one explicitly completing, since the first
		// one already completed, so the zip should too

		assert_eq!(
			notification_collector_1.lock().nth_notification(3),
			&SubscriberNotification::Complete,
			"did not complete"
		);
	}
}

mod overflow {
	use rx_core_notification_store::{QueueOverflowBehavior, QueueOverflowOptions};

	use super::*;

	#[test]
	fn should_properly_complete_after_draining_emitting_two_equal_sized_queues_where_the_second_queue_only_starts_after_the_first_finishes()
	 {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone())
			.with_options(QueueOverflowOptions {
				max_queue_length: 2,
				overflow_behavior: QueueOverflowBehavior::DropOldest,
			})
			.subscribe(destination_1);

		subject_1.next(1); // Expected to be dropped
		subject_1.next(2);
		subject_1.next(3); // 1 drops from the queue
		subject_1.complete();

		assert!(
			!notification_collector_1.lock().nth_notification_exists(0),
			"should not have emitted anything yet"
		);

		subject_2.next("foo");

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((2, "foo")),
			"did not emit the first emission"
		);

		subject_2.next("bar");

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Next((3, "bar")),
			"did not emit the second emission"
		);

		assert_eq!(
			notification_collector_1.lock().nth_notification(2),
			&SubscriberNotification::Complete,
			"did not complete"
		);

		subject_2.next("zed");

		assert!(
			!notification_collector_1.lock().nth_notification_exists(3),
			"should not have emitted anything after it unsubscribed"
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("zip - source_1");

		let source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("zip - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"zip",
			zip(
				source_1
					.clone()
					.finalize(move || source_1_finalized.unsubscribe()),
				source_2
					.clone()
					.finalize(move || source_2_finalized.unsubscribe()),
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
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("zip - source_1");

		let mut source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("zip - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"zip",
			zip(
				source_1
					.clone()
					.finalize(move || source_1_finalized.unsubscribe()),
				source_2
					.clone()
					.finalize(move || source_2_finalized.unsubscribe()),
			),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		source_1.next(1);
		source_2.next("a");
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
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("zip - source_1");

		let source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("zip - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"zip",
			zip(
				source_1
					.clone()
					.finalize(move || source_1_finalized.unsubscribe()),
				source_2
					.clone()
					.finalize(move || source_2_finalized.unsubscribe()),
			),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}
}

mod errors {
	use super::*;

	#[test]
	fn should_error_downstream_when_the_first_observable_errors() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize, &'static str>::default();
		let subject_2 = PublishSubject::<&'static str, &'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.error("error");

		notification_collector.lock().assert_notifications(
			"zip",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}

	#[test]
	fn should_error_downstream_when_the_second_observable_errors() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let subject_1 = PublishSubject::<usize, &'static str>::default();
		let mut subject_2 = PublishSubject::<&'static str, &'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_2.error("error");

		notification_collector.lock().assert_notifications(
			"zip",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}

	#[test]
	fn should_error_downstream_when_the_first_observable_errors_after_seeing_a_value() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize, &'static str>::default();
		let subject_2 = PublishSubject::<&'static str, &'static str>::default();

		let _s = zip(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.next(1);
		subject_1.error("error");

		notification_collector.lock().assert_notifications(
			"zip",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}
}
