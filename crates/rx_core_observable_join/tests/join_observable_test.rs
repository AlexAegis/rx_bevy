use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

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
			[
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
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
				SubscriberNotification::Unsubscribe,
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
				SubscriberNotification::Unsubscribe,
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
				SubscriberNotification::Unsubscribe,
			],
			true,
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

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

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

		let _s = join(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

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
