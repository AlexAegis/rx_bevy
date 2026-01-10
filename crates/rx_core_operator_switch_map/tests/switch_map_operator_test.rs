use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let inner_was_unsubscribed = Arc::new(AtomicUsize::default());
		let inner_was_unsubscribed_clone = inner_was_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<_, usize, TestError>::new_operator_harness("switch_map");
		let observable = harness.create_harness_observable().switch_map(
			move |_next: usize| {
				let inner_was_unsubscribed = inner_was_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.finalize(move || {
					inner_was_unsubscribed.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));

		assert_eq!(inner_was_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let inner_was_unsubscribed = Arc::new(AtomicUsize::default());
		let inner_was_unsubscribed_clone = inner_was_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<_, usize, TestError>::new_operator_harness("switch_map");
		let observable = harness.create_harness_observable().switch_map(
			move |_next: usize| {
				let inner_was_unsubscribed = inner_was_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: true,
					},
					scheduler.clone(),
				)
				.take(1)
				.finalize(move || {
					inner_was_unsubscribed.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert_eq!(inner_was_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let inner_was_unsubscribed = Arc::new(AtomicUsize::default());
		let inner_was_unsubscribed_clone = inner_was_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<_, usize, TestError>::new_operator_harness("switch_map");
		let observable = harness.create_harness_observable().switch_map(
			move |_next: usize| {
				let inner_was_unsubscribed = inner_was_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.finalize(move || {
					inner_was_unsubscribed.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert_eq!(inner_was_unsubscribed.load(Ordering::Relaxed), 1);
	}
}

#[derive(Clone)]
enum Either {
	O1,
	O2,
	O3,
}

#[test]
fn should_switch_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => (0..=2).into_observable(), // They all complete immediately
				Either::O2 => (3..=4).into_observable(),
				Either::O3 => (5..=6).into_observable(),
			},
			|error| error,
		)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("switch_map");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_map - iterators",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_subscribe_to_the_next_observable_immediately_and_unsubscribe_the_previous_subscription() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	notification_collector.lock().assert_is_empty("switch_map");

	inner_2.next(0);

	source.next(Either::O2); // Switches immediately
	inner_3.next(0); // Nothing should happen, haven't yet subscribed

	inner_2.next(1);
	inner_2.next(2);

	notification_collector.lock().assert_notifications(
		"switch_map - first observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	source.next(Either::O2);
	inner_2.next(3);

	source.complete();
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"switch_map - the rest",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_immediately_complete_if_there_are_no_active_subscriptions() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_map",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_immediately_error_by_an_inner_error() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	inner_1.next(1);
	let error = "error";
	inner_3.error(error);

	notification_collector.lock().assert_notifications(
		"switch_map",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after error"
	);
}

#[test]
fn should_immediately_error_by_an_upstream_error() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"switch_map",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after error"
	);
}

#[test]
fn should_compose_and_switch_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let composed = compose_operator().switch_map(
		move |next| match next {
			Either::O1 => (0..=2).into_observable(),
			Either::O2 => (3..=4).into_observable(),
			Either::O3 => (5..=6).into_observable(),
		},
		|error| error,
	);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("switch_map");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_map - iterators",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_be_able_to_subscribe_to_different_observables_if_they_are_erased() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.switch_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone().take(2).erase(),
				Either::O2 => inner_2_clone.clone().map(|i| i * 10).erase(),
				Either::O3 => inner_3_clone.clone().skip(1).erase(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	notification_collector.lock().assert_is_empty("switch_map");

	inner_1.next(1);
	inner_1.next(2); // take 2 completes!
	inner_1.next(99); // nothing!
	source.next(Either::O2); // Switches immediately
	inner_3.next(0); // Nothing should happen, haven't yet subscribed

	inner_2.next(3); // Becomes 30!
	inner_2.next(4); // 40!

	source.next(Either::O3);
	source.complete();

	inner_3.next(99); // Skipped!
	inner_3.next(5);
	inner_3.complete();

	notification_collector.lock().assert_notifications(
		"switch_map - erased",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(30),
			SubscriberNotification::Next(40),
			SubscriberNotification::Next(5),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_be_able_to_execute_inner_teardown_on_switch() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();

	let inner_1_teardown_called = Arc::new(AtomicBool::new(false));
	let inner_1_teardown_called_clone = inner_1_teardown_called.clone();
	let inner_2_teardown_called = Arc::new(AtomicBool::new(false));
	let inner_2_teardown_called_clone = inner_2_teardown_called.clone();
	let inner_3_teardown_called = Arc::new(AtomicBool::new(false));
	let inner_3_teardown_called_clone = inner_3_teardown_called.clone();

	let subscription = source
		.clone()
		.switch_map(
			move |next| {
				let inner_1_teardown = inner_1_teardown_called_clone.clone();
				let inner_2_teardown = inner_2_teardown_called_clone.clone();
				let inner_3_teardown = inner_3_teardown_called_clone.clone();
				match next {
					Either::O1 => inner_1_clone
						.clone()
						.finalize(move || inner_1_teardown.store(true, Ordering::Relaxed))
						.erase(),
					Either::O2 => inner_2_clone
						.clone()
						.finalize(move || inner_2_teardown.store(true, Ordering::Relaxed))
						.erase(),
					Either::O3 => inner_3_clone
						.clone()
						.finalize(move || inner_3_teardown.store(true, Ordering::Relaxed))
						.erase(),
				}
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	notification_collector.lock().assert_is_empty("switch_map");

	inner_1.next(1);
	inner_1.next(2);
	assert!(!inner_1_teardown_called.load(Ordering::Relaxed));
	source.next(Either::O2); // Switches immediately
	assert!(
		inner_1_teardown_called.load(Ordering::Relaxed),
		"inner teardown 1 not called!"
	);
	inner_2.next(3);
	inner_2.next(4);

	source.next(Either::O3);
	assert!(
		inner_2_teardown_called.load(Ordering::Relaxed),
		"inner teardown 2 not called!"
	);
	source.complete();
	inner_3.next(5);
	inner_3.complete();
	assert!(
		inner_3_teardown_called.load(Ordering::Relaxed),
		"inner teardown 3 not called!"
	);

	notification_collector.lock().assert_notifications(
		"switch_map - inner teardown",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_be_able_to_work_with_an_interval_as_the_inner_subscription() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let mut source = PublishSubject::<usize>::default();

	let inner_was_unsubscribed = Arc::new(AtomicUsize::default());
	let inner_was_unsubscribed_clone = inner_was_unsubscribed.clone();
	let mut subscription = source
		.clone()
		.enumerate()
		.switch_map(
			move |_| {
				let inner_was_unsubscribed = inner_was_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.finalize(move || {
					inner_was_unsubscribed.fetch_add(1, Ordering::Relaxed);
				})
			},
			Never::map_into(),
		)
		.subscribe(destination);

	source.next(0);

	assert_eq!(inner_was_unsubscribed.load(Ordering::Relaxed), 0);

	notification_collector.lock().assert_is_empty("switch_map");

	executor.tick(Duration::from_millis(400));

	notification_collector.lock().assert_notifications(
		"switch_map",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	source.complete();

	executor.tick(Duration::from_millis(400));

	subscription.unsubscribe();

	assert_eq!(inner_was_unsubscribed.load(Ordering::Relaxed), 1);

	executor.tick(Duration::from_millis(400));

	notification_collector.print();
	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}
