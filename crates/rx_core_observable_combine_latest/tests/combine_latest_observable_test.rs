use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_only_emit_after_both_observables_emitted_even_if_its_not_in_order() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<&'static str>::default();

	let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

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

	mod when_one_input_observable_completes {
		use super::*;

		#[test]
		fn should_just_unsubscribe() {
			let destination_1 = MockObserver::default();
			let notification_collector_1 = destination_1.get_notification_collector();

			let mut subject_1 = PublishSubject::<usize>::default();
			let subject_2 = PublishSubject::<&'static str>::default();

			let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

			subject_1.complete();

			assert_eq!(
				notification_collector_1.lock().nth_notification(0),
				&SubscriberNotification::Unsubscribe,
				"did not unsubscribe"
			);
		}
	}

	#[test]
	fn should_unsubscribe_but_not_complete_when_one_of_the_observables_unsubscribe_without_emitting_values_before()
	 {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Unsubscribe,
			"did not unsubscribe"
		);
	}

	#[test]
	fn should_not_unsubscribe_when_one_input_observable_unsubscribes_with_values_already_emitted_while_the_other_is_waiting()
	 {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_1.unsubscribe();

		assert!(
			!notification_collector_1.lock().nth_notification_exists(0),
			"An event was observed when none should have"
		);

		subject_2.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Unsubscribe,
			"Did not unsubscribe"
		);
	}

	#[test]
	fn should_not_complete_when_one_input_observable_completes_with_values_already_emitted_while_the_other_is_waiting()
	 {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_1.complete();

		assert!(
			!notification_collector_1.lock().nth_notification_exists(0),
			"An event was observed when none should have"
		);

		subject_2.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Complete,
			"Did not unsubscribe"
		);
	}
}

mod after_primed {
	use super::*;
	#[test]
	fn should_only_complete_after_both_observables_completed() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_2.next("hello");
		subject_1.next(1);

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "hello")),
			"the first emission isn't correct"
		);

		subject_1.complete();

		assert!(
			!notification_collector_1.lock().nth_notification_exists(1),
			"should not have completed yet, the other observable can still cause emissions!"
		);

		subject_2.next("bello");

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Next((1, "bello")),
			"the second emission isn't correct"
		);

		subject_2.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(2),
			&SubscriberNotification::Complete,
			"the third emission isn't a complete"
		);
	}

	#[test]
	fn should_not_complete_when_one_input_observable_completes() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_2.next("hello");
		subject_1.complete();

		notification_collector_1.lock().assert_notifications(
			"publish_subject destination",
			0,
			[SubscriberNotification::Next((1, "hello"))],
			true,
		);

		subject_2.complete();

		notification_collector_1.lock().assert_notifications(
			"publish_subject destination",
			1,
			[SubscriberNotification::Complete],
			true,
		);
	}

	#[test]
	fn should_not_complete_when_only_one_observable_completed_and_the_other_just_unsubscribed() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_2.next("hello");
		subject_1.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "hello")),
			"Did not receive the first emission"
		);

		assert!(
			!notification_collector_1.lock().nth_notification_exists(1),
			"An event was observed when none should have"
		);

		subject_2.unsubscribe();

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"Did not unsubscribe"
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut source_1 = PublishSubject::<usize, MockError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown =
			source_1_finalized.add_tracked_teardown("combine_latest - source_1");

		let source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown =
			source_2_finalized.add_tracked_teardown("combine_latest - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"combine_latest",
			combine_latest(
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
		let source_1_tracked_teardown =
			source_1_finalized.add_tracked_teardown("combine_latest - source_1");

		let mut source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown =
			source_2_finalized.add_tracked_teardown("combine_latest - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"combine_latest",
			combine_latest(
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
		let source_1_tracked_teardown =
			source_1_finalized.add_tracked_teardown("combine_latest - source_1");

		let source_2 = PublishSubject::<&'static str, MockError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown =
			source_2_finalized.add_tracked_teardown("combine_latest - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), MockError>::new_with_source(
			"combine_latest",
			combine_latest(
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

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.error("error");

		notification_collector.lock().assert_notifications(
			"combine_latest",
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

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_2.error("error");

		notification_collector.lock().assert_notifications(
			"combine_latest",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}
}
