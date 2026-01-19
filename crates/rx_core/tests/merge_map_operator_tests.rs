use std::{
	sync::{
		Arc,
		atomic::{AtomicUsize, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Clone)]
enum Either {
	O1,
	O2,
	O3,
}

#[test]
fn should_merge_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.merge_map(
			move |next| match next {
				Either::O1 => (0..=2).into_observable(),
				Either::O2 => (3..=4).into_observable(),
				Either::O3 => (5..=6).into_observable(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_all - iterators",
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
fn should_subscribe_to_all_of_the_inner_observables_if_the_limit_allows_it() {
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
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_is_empty("merge_map");

	inner_2.next(0);
	inner_3.next(1);
	inner_1.next(2);
	inner_1.next(3);

	notification_collector.lock().assert_notifications(
		"merge_map - first observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
		],
		true,
	);

	inner_1.complete();
	inner_2.next(4);
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"merge_map - the rest",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
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
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_map",
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
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	inner_1.next(1);
	let error = "error";
	inner_1.error(error);

	notification_collector.lock().assert_notifications(
		"merge_map",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
		],
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
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"merge_map",
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
fn should_subscribe_to_as_many_of_the_inner_observables_as_the_limit_allows_it() {
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
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			2,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	inner_3.next(1); // Nothing should happen as only 2 can be subscribed at a time

	notification_collector.lock().assert_is_empty("merge_map");

	inner_2.next(0);
	inner_1.next(1);
	inner_1.next(2);

	notification_collector.lock().assert_notifications(
		"merge_map - first observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	inner_3.next(3);
	inner_3.complete();
	inner_2.next(4);
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"merge_map - the rest",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
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
fn should_compose_and_merge_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let composed = compose_operator().merge_map(
		move |next| match next {
			Either::O1 => (0..=2).into_observable(),
			Either::O2 => (3..=4).into_observable(),
			Either::O3 => (5..=6).into_observable(),
		},
		usize::MAX,
		|error| error,
	);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_all - iterators",
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
		let inner_unsubscribed_clone = inner_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("merge_map");
		let observable = harness.create_harness_observable().merge_map(
			move |_next: usize| {
				let counter = inner_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.finalize(move || {
					counter.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			usize::MAX,
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let inner_unsubscribed = Arc::new(AtomicUsize::default());
		let inner_unsubscribed_clone = inner_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("merge_map");
		let observable = harness.create_harness_observable().merge_map(
			move |_next: usize| {
				let counter = inner_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(10),
						max_emissions_per_tick: 10,
						start_on_subscribe: true,
					},
					scheduler.clone(),
				)
				.take(1)
				.finalize(move || {
					counter.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			usize::MAX,
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let inner_unsubscribed = Arc::new(AtomicUsize::default());
		let inner_unsubscribed_clone = inner_unsubscribed.clone();

		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("merge_map");
		let observable = harness.create_harness_observable().merge_map(
			move |_next: usize| {
				let counter = inner_unsubscribed_clone.clone();
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(200),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.finalize(move || {
					counter.fetch_add(1, Ordering::Relaxed);
				})
				.map_never()
			},
			usize::MAX,
			|error| error,
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert_eq!(inner_unsubscribed.load(Ordering::Relaxed), 1);
	}
}
