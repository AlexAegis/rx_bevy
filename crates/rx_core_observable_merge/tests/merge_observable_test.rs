use rx_core::prelude::*;
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
