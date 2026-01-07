use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_only_emit_after_both_observables_emitted_even_if_its_not_in_order() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<&'static str>::default();

	let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

	subject_2.next("hello");

	subject_1.next(1);

	notification_collector.lock().assert_notifications(
		"combine_changes",
		0,
		[
			SubscriberNotification::Next((Change::None, Change::JustUpdated("hello"))),
			SubscriberNotification::Next((Change::JustUpdated(1), Change::Latest("hello"))),
		],
		true,
	);
}

mod before_primed {
	use super::*;

	#[test]
	fn should_not_complete_when_only_one_of_the_observables_complete_and_not_unsubscribe() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.complete();

		notification_collector
			.lock()
			.assert_notifications("combine_changes", 0, [], true);
	}

	#[test]
	fn should_be_possible_to_emit_even_if_one_of_the_input_observables_had_immediately_completed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_2 = PublishSubject::<usize>::default();

		let _s = combine_changes(empty(), subject_2.clone()).subscribe(destination);

		subject_2.next(1);
		subject_2.complete();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::None, Change::JustUpdated(1))),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_be_possible_to_emit_even_if_one_of_the_input_observables_had_immediately_closed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_2 = PublishSubject::<usize>::default();

		let _s = combine_changes(closed(), subject_2.clone()).subscribe(destination);

		subject_2.next(1);
		subject_2.complete();

		// Since not all input observables have completed, but one cancelled,
		// there is no completion signal.
		// TODO: This is the behavior for CombineLatest in rxjs, but since cancelling just one input observable does not cancel the entire combineLatest, it should not prevent a later completion either, as the last observed value can still be replayed when other observables emit. So this behavior might change later.

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::None, Change::JustUpdated(1))),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn should_unsubscribe_when_both_of_the_observables_unsubscribe_without_emitting_values_before()
	{
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.unsubscribe();
		subject_2.unsubscribe();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn should_not_unsubscribe_when_one_input_observable_unsubscribes_with_values_already_emitted_while_the_other_is_waiting()
	 {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.next(1);
		subject_1.unsubscribe();
		subject_2.unsubscribe();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::JustUpdated(1), Change::None)),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn should_not_complete_when_one_input_observable_completes_with_values_already_emitted_while_the_other_is_waiting()
	 {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.next(1);
		subject_1.complete();
		subject_2.complete();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::JustUpdated(1), Change::None)),
				SubscriberNotification::Complete,
			],
			true,
		);
	}
}

mod after_primed {
	use super::*;
	#[test]
	fn should_only_complete_after_both_observables_completed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_2.next("hello");
		subject_1.next(1);
		subject_1.complete();
		subject_2.next("bello");
		subject_2.complete();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::None, Change::JustUpdated("hello"))),
				SubscriberNotification::Next((Change::JustUpdated(1), Change::Latest("hello"))),
				SubscriberNotification::Next((Change::Latest(1), Change::JustUpdated("bello"))),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_not_complete_when_one_input_observable_completes() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.next(1);
		subject_2.next("hello");
		subject_1.complete();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::JustUpdated(1), Change::None)),
				SubscriberNotification::Next((Change::Latest(1), Change::JustUpdated("hello"))),
			],
			true,
		);
	}

	#[test]
	fn should_not_complete_when_only_one_observable_completed_and_the_other_just_unsubscribed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut subject_1 = PublishSubject::<usize>::default();
		let mut subject_2 = PublishSubject::<&'static str>::default();

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_1.next(1);
		subject_2.next("hello");
		subject_1.complete();
		subject_2.unsubscribe();

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[
				SubscriberNotification::Next((Change::JustUpdated(1), Change::None)),
				SubscriberNotification::Next((Change::Latest(1), Change::JustUpdated("hello"))),
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

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination_1);

		subject_1.error("error");

		notification_collector_1.lock().assert_notifications(
			"combine_changes",
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

		let _s = combine_changes(subject_1.clone(), subject_2.clone()).subscribe(destination);

		subject_2.error("error");

		notification_collector.lock().assert_notifications(
			"combine_changes",
			0,
			[SubscriberNotification::Error("error")],
			true,
		);
	}
}
