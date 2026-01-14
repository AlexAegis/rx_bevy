use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

mod before_primed {
	use super::*;

	#[test]
	fn should_only_complete_when_both_sources_complete() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.complete();
		subject_2.complete();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[SubscriberNotification::Complete],
			true,
		);
	}

	#[test]
	fn should_only_unsubscribe_when_only_the_first_source_completes() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.complete();
		subject_2.unsubscribe();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn should_only_unsubscribe_when_only_the_second_source_completes() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.unsubscribe();
		subject_2.complete();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn should_only_unsubscribe_when_none_of_the_sources_completes() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.unsubscribe();
		subject_2.unsubscribe();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}
}

mod after_primed {
	use super::*;

	#[test]
	fn should_emit_the_value_pair_once_both_complete() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_2.next("hello");
		subject_1.complete();
		subject_2.complete();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[
				SubscriberNotification::Next((1, "hello")),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_emit_the_value_pair_even_when_the_complete_sequentially() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.next(1);
		subject_1.complete();
		subject_2.next("hello");
		subject_2.complete();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[
				SubscriberNotification::Next((1, "hello")),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_emit_the_value_pair_even_when_the_complete_sequentially_in_reverse() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_2.next("hello");
		subject_2.complete();

		subject_1.next(1);
		subject_1.complete();

		notification_collector_1.lock().assert_notifications(
			"join",
			0,
			[
				SubscriberNotification::Next((1, "hello")),
				SubscriberNotification::Complete,
			],
			true,
		);
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

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.error("error");

		notification_collector.lock().assert_notifications(
			"join",
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

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_2.error("error");

		notification_collector.lock().assert_notifications(
			"join",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut source_1 = PublishSubject::<usize, TestError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("join - source_1");

		let source_2 = PublishSubject::<&'static str, TestError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("join - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), TestError>::new_with_source(
			"join",
			join(
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

		source_1.error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));

		source_1_tracked_teardown.assert_was_torn_down();
		source_2_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut source_1 = PublishSubject::<usize, TestError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("join - source_1");

		let mut source_2 = PublishSubject::<&'static str, TestError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("join - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), TestError>::new_with_source(
			"join",
			join(
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
		let source_1 = PublishSubject::<usize, TestError>::default();
		let mut source_1_finalized = SharedSubscription::default();
		let source_1_tracked_teardown = source_1_finalized.add_tracked_teardown("join - source_1");

		let source_2 = PublishSubject::<&'static str, TestError>::default();
		let mut source_2_finalized = SharedSubscription::default();
		let source_2_tracked_teardown = source_2_finalized.add_tracked_teardown("join - source_2");

		let mut harness = TestHarness::<_, (usize, &'static str), TestError>::new_with_source(
			"join",
			join(
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
