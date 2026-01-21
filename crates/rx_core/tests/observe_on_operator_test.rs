use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::{Observable, SharedSubscriber};
use rx_core_testing::prelude::*;

#[test]
fn should_schedule_a_next_emission_on_tick() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.observe_on(scheduler.clone())
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
		"No notifications should've been observed yet"
	);

	executor.tick(Duration::from_millis(0));

	notification_collector.lock().assert_notifications(
		"observe_on",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

// This actually tests the `is_closed` function of `observe_on` which should
// immediately give feedback to upstream that it is closed, which is used
// for things like stopping an iterator early.
#[test]
fn should_not_finish_the_iterator_when_closed_early_and_downstream_is_observed_on() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let mock_destination = MockObserver::<i32>::default();
	let notification_collector = mock_destination.get_notification_collector();

	let tracked_iterator = TrackedIterator::new(1..=5);
	let tracked_data = tracked_iterator.get_tracking_data_ref();

	let mut upstream_teardown_tracker_subscription = SharedSubscription::default();
	let upstream_teardown_tracker = upstream_teardown_tracker_subscription
		.add_tracked_teardown("iterator - take(2) - observe_on");
	let mut source = tracked_iterator
		.into_observable()
		.finalize(move || upstream_teardown_tracker_subscription.unsubscribe())
		.take(2)
		.observe_on(scheduler);
	let subscription = source.subscribe(mock_destination);

	assert!(!tracked_data.is_finished(0), "Should not reach its end!");
	assert!(notification_collector.lock().is_empty());

	executor.tick(Duration::from_millis(0));

	upstream_teardown_tracker.assert_was_torn_down();
	notification_collector.lock().assert_notifications(
		"iterator - take(2) - observe_on",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn should_schedule_multiple_next_emissions_on_tick() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.observe_on(scheduler.clone())
		.subscribe(destination);

	assert!(executor.is_empty(), "No work should've been scheduled yet");

	source.next(1);
	source.next(2);

	notification_collector.lock().assert_is_empty("observe_on");

	executor.tick(Duration::from_millis(0));

	notification_collector.lock().assert_notifications(
		"observe_on",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

#[test]
fn should_skip_scheduled_next_when_downstream_closes_before_tick() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();
	let shared_destination = SharedSubscriber::new(destination);
	let mut shared_destination_unsubscribe = shared_destination.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.observe_on(scheduler.clone())
		.subscribe(shared_destination.clone());

	source.next(1);

	assert!(
		scheduler.lock().has_actions(),
		"Work should be scheduled now"
	);

	let destination_lock = shared_destination.lock();
	shared_destination_unsubscribe.unsubscribe();

	executor.tick(Duration::from_millis(0));
	assert!(executor.is_empty(), "All work should be drained by now");

	drop(destination_lock);
	shared_destination_unsubscribe.unsubscribe();

	notification_collector.lock().assert_notifications(
		"observe_on",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

mod error {
	use super::*;

	#[test]
	fn should_ignore_observe_on_for_errors_and_error_instantly_and_cancel_scheduled_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);

		executor.tick(Duration::from_millis(0)); // Tick to drain any scheduled work
		assert!(executor.is_empty(), "All work should be drained!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_cancel_work_when_errored_even_if_the_tick_would_execute_the_scheduled_work() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);

		executor.tick(Duration::from_millis(0)); // Tick to drain any scheduled work
		assert!(executor.is_empty(), "All work should be drained!");

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);

		assert!(subscription.is_closed());
	}
}

mod complete {
	use super::*;

	#[test]
	fn should_complete_after_a_tick_when_there_are_scheduled_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		source.complete();

		notification_collector.lock().assert_is_empty("observe_on");

		executor.tick(Duration::from_millis(0));

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_complete_when_there_were_no_scheduled_emissions_yet() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.complete();

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_complete_after_tick_when_emission_had_passed_and_complete_was_received_later() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");
		source.next(1);
		source.complete();
		source.next(2);

		notification_collector.lock().assert_is_empty("observe_on");

		executor.tick(Duration::from_millis(0));

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
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
	fn should_unsubscribe_after_a_tick_when_there_are_scheduled_emissions() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.next(1);
		source.unsubscribe();

		notification_collector.lock().assert_is_empty("observe_on");

		executor.tick(Duration::from_millis(0));

		notification_collector.lock().assert_notifications(
			"observe_on",
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
	fn should_immediately_unsubscribe_when_there_were_no_scheduled_emissions_yet() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");

		source.unsubscribe();

		notification_collector.lock().assert_notifications(
			"observe_on",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);

		assert!(executor.is_empty(), "All work should've been executed!");

		assert!(subscription.is_closed());
	}

	#[test]
	fn should_unsubscribe_after_tick_when_emission_had_passed_and_unsubscribe_was_received_later() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.observe_on(scheduler.clone())
			.subscribe(destination);

		assert!(executor.is_empty(), "No work should've been scheduled yet");
		source.next(1);
		source.unsubscribe();
		source.next(2);

		notification_collector.lock().assert_is_empty("observe_on");

		executor.tick(Duration::from_millis(0));

		notification_collector.lock().assert_notifications(
			"observe_on",
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
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator().observe_on(scheduler.clone());

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

	executor.tick(Duration::from_millis(0));

	notification_collector.lock().assert_notifications(
		"observe_on",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

mod contracts {
	use super::*;

	/// Errors are instantenous and not delayed, hence the missing tick call.
	#[test]
	fn rx_contract_closed_after_error() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new("observe_on");

		let observable = harness
			.create_harness_observable()
			.observe_on(scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error("error");
		harness.assert_terminal_notification(SubscriberNotification::Error("error"));

		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new("observe_on");

		let observable = harness
			.create_harness_observable()
			.observe_on(scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		executor.tick(Duration::from_millis(0));
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new("observe_on");

		let observable = harness
			.create_harness_observable()
			.observe_on(scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		executor.tick(Duration::from_millis(0));
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert!(executor.is_empty());
	}
}
