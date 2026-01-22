use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::SharedSubscriber;
use rx_core_testing::prelude::*;

#[test]
fn should_schedule_subscription_on_tick() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.subscribe_on(scheduler.clone())
		.subscribe(destination);

	assert!(scheduler.lock().has_actions(), "Work should be scheduled");

	source.next(1);
	notification_collector
		.lock()
		.assert_is_empty("subscribe_on");

	executor.tick(Duration::from_millis(0));

	source.next(2);

	notification_collector.lock().assert_notifications(
		"subscribe_on",
		0,
		[SubscriberNotification::Next(2)],
		true,
	);

	subscription.unsubscribe();
}

#[test]
fn should_delay_subscription_until_the_delay_elapses() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.subscribe_on_with_delay(Duration::from_millis(1000), scheduler.clone())
		.subscribe(destination);

	source.next(1);
	executor.tick(Duration::from_millis(999));
	source.next(2);

	notification_collector
		.lock()
		.assert_is_empty("subscribe_on");

	executor.tick(Duration::from_millis(1));

	source.next(3);

	notification_collector.lock().assert_notifications(
		"subscribe_on",
		0,
		[SubscriberNotification::Next(3)],
		true,
	);

	subscription.unsubscribe();
}

#[test]
fn should_cancel_scheduled_subscription_when_unsubscribed_early() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.subscribe_on_with_delay(Duration::from_millis(1000), scheduler.clone())
		.subscribe(destination);

	subscription.unsubscribe();
	executor.tick(Duration::from_millis(1000));

	source.next(1);

	notification_collector.lock().assert_notifications(
		"subscribe_on",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	assert!(executor.is_empty());
}

#[test]
fn should_skip_scheduled_subscription_when_destination_closed_before_tick() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();
	let shared_destination = SharedSubscriber::new(destination);
	let mut shared_destination_unsubscribe = shared_destination.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.subscribe_on(scheduler.clone())
		.subscribe(shared_destination.clone());

	source.next(1);
	notification_collector
		.lock()
		.assert_is_empty("subscribe_on - before tick");

	assert!(scheduler.lock().has_actions(), "Work should be scheduled");

	let destination_lock = shared_destination.lock();
	shared_destination_unsubscribe.unsubscribe();

	executor.tick(Duration::from_millis(0));
	assert!(executor.is_empty(), "All work should be drained by now");

	drop(destination_lock);
	shared_destination_unsubscribe.unsubscribe();

	notification_collector.lock().assert_notifications(
		"subscribe_on",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_unsubscribe_from_upstream_when_destination_closes_during_subscription() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let subscription = empty()
		.subscribe_on(scheduler.clone())
		.subscribe(destination);

	executor.tick(Duration::from_millis(0));

	notification_collector.lock().assert_notifications(
		"subscribe_on",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"subscribe_on",
		);

		let observable = harness
			.create_harness_observable()
			.subscribe_on(scheduler.clone());
		harness.subscribe_to(observable);
		executor.tick(Duration::from_millis(0));
		harness.source().next(1);
		harness.source().error("error");
		harness.assert_terminal_notification(SubscriberNotification::Error("error"));

		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"subscribe_on",
		);

		let observable = harness
			.create_harness_observable()
			.subscribe_on(scheduler.clone());
		harness.subscribe_to(observable);
		executor.tick(Duration::from_millis(0));
		harness.source().next(1);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"subscribe_on",
		);

		let observable = harness
			.create_harness_observable()
			.subscribe_on(scheduler.clone());
		harness.subscribe_to(observable);
		executor.tick(Duration::from_millis(0));
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert!(executor.is_empty());
	}
}
