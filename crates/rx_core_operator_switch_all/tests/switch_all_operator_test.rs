use std::{
	sync::{
		Arc,
		atomic::{AtomicUsize, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_switch_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.switch_all(|error| error)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("switch_all");

	source.next((0..=2).into_observable()); // They all complete immediately
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_all - iterators",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let subscription = source
		.clone()
		.switch_all(|error| error)
		.subscribe(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	source.next(inner_1.clone());
	notification_collector.lock().assert_is_empty("switch_all");

	inner_2.next(0);

	source.next(inner_2.clone()); // Switches immediately
	inner_3.next(0); // Nothing should happen, haven't yet subscribed

	inner_2.next(1);
	inner_2.next(2);

	notification_collector.lock().assert_notifications(
		"switch_all - first two observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	source.next(inner_2.clone());
	inner_2.next(3);

	source.complete();
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"switch_all - the rest",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let subscription = source
		.clone()
		.switch_all(|error| error)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_all",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.switch_all(|error| error)
		.subscribe(destination);

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	source.complete();

	inner_1.next(1);
	let error = "error";
	inner_3.error(error);

	notification_collector.lock().assert_notifications(
		"switch_all",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let subscription = source
		.clone()
		.switch_all(|error| error)
		.subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"switch_all",
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

	let composed = compose_operator().switch_all(|error| error);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("switch_all");

	source.next((0..=2).into_observable());
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
	source.complete();

	notification_collector.lock().assert_notifications(
		"switch_all - iterators",
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

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let inner_unsubscribed = Arc::new(AtomicUsize::default());
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<
			TestSubject<ErasedObservable<usize, MockError>, MockError>,
			usize,
			MockError,
		>::new("switch_all");
		let observable = harness
			.create_harness_observable()
			.switch_all(|error| error);
		harness.subscribe_to(observable);

		let source_scheduler = scheduler.clone();
		let source_counter = inner_unsubscribed.clone();
		harness.source().next(
			interval(
				IntervalObservableOptions {
					duration: Duration::from_millis(200),
					max_emissions_per_tick: 10,
					start_on_subscribe: false,
				},
				source_scheduler,
			)
			.finalize(move || {
				source_counter.fetch_add(1, Ordering::Relaxed);
			})
			.map_error(|_never| MockError)
			.erase(),
		);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let inner_unsubscribed = Arc::new(AtomicUsize::default());
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<
			TestSubject<ErasedObservable<usize, MockError>, MockError>,
			usize,
			MockError,
		>::new("switch_all");
		let observable = harness
			.create_harness_observable()
			.switch_all(|error| error);
		harness.subscribe_to(observable);

		let source_scheduler = scheduler.clone();
		let source_counter = inner_unsubscribed.clone();
		harness.source().next(
			interval(
				IntervalObservableOptions {
					duration: Duration::from_millis(10),
					max_emissions_per_tick: 10,
					start_on_subscribe: true,
				},
				source_scheduler,
			)
			.take(1)
			.finalize(move || {
				source_counter.fetch_add(1, Ordering::Relaxed);
			})
			.map_error(|_never| MockError)
			.erase(),
		);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let inner_unsubscribed = Arc::new(AtomicUsize::default());
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness = TestHarness::<
			TestSubject<ErasedObservable<usize, MockError>, MockError>,
			usize,
			MockError,
		>::new("switch_all");
		let observable = harness
			.create_harness_observable()
			.switch_all(|error| error);
		harness.subscribe_to(observable);

		let source_scheduler = scheduler.clone();
		let source_counter = inner_unsubscribed.clone();
		harness.source().next(
			interval(
				IntervalObservableOptions {
					duration: Duration::from_millis(200),
					max_emissions_per_tick: 10,
					start_on_subscribe: false,
				},
				source_scheduler,
			)
			.finalize(move || {
				source_counter.fetch_add(1, Ordering::Relaxed);
			})
			.map_error(|_never| MockError)
			.erase(),
		);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}
}
