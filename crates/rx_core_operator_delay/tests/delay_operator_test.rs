use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::Observable;

#[test]
fn should_delay_a_next_emission_by_the_specified_amount_of_time() {
	let mut executor = MockExecutor::default();
	let mut scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.delay(Duration::from_millis(1000), scheduler.clone())
		.subscribe(destination);

	assert!(executor.is_empty(), "No work should've been scheduled yet");

	source.next(1);

	assert!(
		scheduler.lock().has_actions(),
		"Work should be scheduled now"
	);
	assert!(
		executor.is_empty(),
		"But not yet drained by the executor as it hadn't ticked yet."
	);
	assert!(
		notification_collector.lock().is_empty(),
		"No notifications should've observed yet"
	);
	executor.tick(Duration::from_millis(999));
	assert!(
		!executor.is_empty(),
		"Work is scheduled now, but insufficient time had passed"
	);
	assert!(
		notification_collector.lock().is_empty(),
		"No notifications should've observed yet, insufficient time had passed"
	);

	executor.tick(Duration::from_millis(1));

	notification_collector.lock().assert_notifications(
		"delay",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

#[test]
fn should_delay_multiple_next_emissions_by_the_specified_amount_of_time() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.delay(Duration::from_millis(1000), scheduler.clone())
		.subscribe(destination);

	assert!(executor.is_empty(), "No work should've been scheduled yet");

	source.next(1);
	executor.tick(Duration::from_millis(500));
	source.next(2);
	executor.tick(Duration::from_millis(500));

	notification_collector.lock().assert_notifications(
		"delay",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(!executor.is_empty(), "Work should still be scheduled!");

	executor.tick(Duration::from_millis(500));

	notification_collector.lock().assert_notifications(
		"delay",
		1,
		[SubscriberNotification::Next(2)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

mod error {
	use super::*;

	#[test]
	fn should_ignore_delay_for_errors_and_error_instantly_and_cancel_existing_delayed_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		executor.tick(Duration::from_millis(500));
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Error(error),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(
			!executor.is_empty(),
			"Work should still be scheduled to be executed!"
		);

		executor.tick(Duration::from_millis(0)); // Tick to drain the scheduler
		assert!(executor.is_empty(), "Work should've been cancelled!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_cancel_work_when_errored_even_if_the_tick_would_execute_the_already_scheduled_work() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		executor.tick(Duration::from_millis(500));
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Error(error),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(
			!executor.is_empty(),
			"Work should still be scheduled to be executed!"
		);

		executor.tick(Duration::from_millis(1000)); // Tick to drain the scheduler
		assert!(executor.is_empty(), "Work should've been cancelled!");

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Error(error),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(subscription.is_closed());
	}
}

mod complete {
	use super::*;

	#[test]
	fn should_complete_after_a_delay_when_there_are_delayed_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		source.complete();

		notification_collector.lock().assert_is_empty("delay");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_complete_when_there_were_no_delayed_emissions_yet() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.complete();

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_complete_when_an_emission_had_passed_and_complete_was_received_later() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");
		source.next(1);
		executor.tick(Duration::from_millis(500));
		source.complete(); // This should not be delayed a full 1000ms!
		source.next(2);

		notification_collector.lock().assert_is_empty("delay");

		executor.tick(Duration::from_millis(500));

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}
}

mod unsubscribe {
	use super::*;

	#[test]
	fn should_unsubscribe_after_a_delay_when_there_are_delayed_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		source.unsubscribe();

		notification_collector.lock().assert_is_empty("delay");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_unsubscribe_when_there_were_no_delayed_emissions_yet() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.unsubscribe();

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_unsubscribe_when_an_emission_had_passed_and_unsubscribe_was_received_later()
	 {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.delay(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");
		source.next(1);
		executor.tick(Duration::from_millis(500));
		source.unsubscribe(); // This should not be delayed a full 1000ms!
		source.next(2);

		notification_collector.lock().assert_is_empty("delay");

		executor.tick(Duration::from_millis(500));

		notification_collector.lock().assert_notifications(
			"delay",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}
}

#[test]
fn should_compose() {
	let mut executor = MockExecutor::default();
	let mut scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator().delay(Duration::from_millis(1000), scheduler.clone());

	let mut subscription = source.clone().pipe(composed).subscribe(destination);

	assert!(executor.is_empty(), "No work should've been scheduled yet");

	source.next(1);

	assert!(
		scheduler.lock().has_actions(),
		"Work should be scheduled now"
	);
	assert!(
		executor.is_empty(),
		"But not yet drained by the executor as it hadn't ticked yet."
	);
	assert!(
		notification_collector.lock().is_empty(),
		"No notifications should've observed yet"
	);
	executor.tick(Duration::from_millis(999));
	assert!(
		!executor.is_empty(),
		"Work is scheduled now, but insufficient time had passed"
	);
	assert!(
		notification_collector.lock().is_empty(),
		"No notifications should've observed yet, insufficient time had passed"
	);

	executor.tick(Duration::from_millis(1));

	notification_collector.lock().assert_notifications(
		"delay",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}
