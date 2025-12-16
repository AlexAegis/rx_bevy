use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

use crate::observable_fn::combine_latest;

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

	#[test]
	fn should_complete_even_when_only_one_of_the_observables_complete() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Complete,
			"did not complete"
		);
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

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Next((1, "hello")),
			"Did not receive the first emission"
		);

		assert!(
			!notification_collector_1.lock().nth_notification_exists(1),
			"An event was observed when none should have"
		);

		subject_2.complete();

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Complete,
			"Did not unsubscribe"
		);
	}
}

mod errors {
	use super::*;

	#[test]
	fn should_error_downstream_when_the_first_observable_errors() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize, &'static str>::default();
		let subject_2 = PublishSubject::<&'static str, &'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.error("error");

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Error("error"),
			"Did not receive the first emission"
		);

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"Did not unsubscribe"
		);
	}

	#[test]
	fn should_error_downstream_when_the_second_observable_errors() {
		let destination_1 = MockObserver::default();
		let notification_collector_1 = destination_1.get_notification_collector();

		let subject_1 = PublishSubject::<usize, &'static str>::default();
		let mut subject_2 = PublishSubject::<&'static str, &'static str>::default();

		let _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_2.error("error");

		assert_eq!(
			notification_collector_1.lock().nth_notification(0),
			&SubscriberNotification::Error("error"),
			"Did not receive the first emission"
		);

		assert_eq!(
			notification_collector_1.lock().nth_notification(1),
			&SubscriberNotification::Unsubscribe,
			"Did not unsubscribe"
		);
	}
}
