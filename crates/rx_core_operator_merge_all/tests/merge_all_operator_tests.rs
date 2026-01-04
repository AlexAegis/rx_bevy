use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_merge_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.merge_all(usize::MAX, |error| error)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next((0..=2).into_observable());
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(usize::MAX, |error| error)
		.subscribe(destination);

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	source.complete();

	notification_collector.lock().assert_is_empty("merge_all");

	inner_2.next(0);
	inner_3.next(1);
	inner_1.next(2);
	inner_1.next(3);

	notification_collector.lock().assert_notifications(
		"merge_all - first observable",
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
		"merge_all - the rest",
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
fn should_subscribe_to_as_many_of_the_inner_observables_as_the_limit_allows_it() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(2, |error| error)
		.subscribe(destination);

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	source.complete();

	inner_3.next(1); // Nothing should happen as only 2 can be subscribed at a time

	notification_collector.lock().assert_is_empty("merge_all");

	inner_2.next(0);
	inner_1.next(1);
	inner_1.next(2);

	notification_collector.lock().assert_notifications(
		"merge_all - first observable",
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
		"merge_all - the rest",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(2, |error| error)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_all",
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
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(usize::MAX, |error| error)
		.subscribe(destination);

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	source.complete();

	inner_1.next(1);
	let error = "error";
	inner_1.error(error);

	notification_collector.lock().assert_notifications(
		"merge_all",
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

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(usize::MAX, |error| error)
		.subscribe(destination);

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"merge_all",
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
fn should_compose_and_merge_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let composed = compose_operator().merge_all(usize::MAX, |error| error);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next((0..=2).into_observable());
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
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
fn should_execute_all_active_inner_teardowns_when_one_errors() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source =
		PublishSubject::<ErasedObservable<usize, &'static str>, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.merge_all(usize::MAX, |error| error)
		.subscribe(destination);

	let inner_1_teardown_was_called = Arc::new(AtomicBool::new(false));
	let inner_1_teardown_was_called_finalze = inner_1_teardown_was_called.clone();
	let inner_2_teardown_was_called = Arc::new(AtomicBool::new(false));
	let inner_2_teardown_was_called_finalze = inner_2_teardown_was_called.clone();
	let inner_3_teardown_was_called = Arc::new(AtomicBool::new(false));
	let inner_3_teardown_was_called_finalze = inner_3_teardown_was_called.clone();

	source.next(
		inner_1
			.clone()
			.finalize(move || inner_1_teardown_was_called_finalze.store(true, Ordering::Relaxed))
			.erase(),
	);
	source.next(
		inner_2
			.clone()
			.finalize(move || inner_2_teardown_was_called_finalze.store(true, Ordering::Relaxed))
			.erase(),
	);
	source.next(
		inner_3
			.clone()
			.finalize(move || inner_3_teardown_was_called_finalze.store(true, Ordering::Relaxed))
			.erase(),
	);
	source.complete();

	notification_collector.lock().assert_is_empty("merge_all");

	inner_2.next(0);
	inner_3.next(1);
	inner_1.next(2);
	inner_1.next(3);
	let error = "error";
	inner_2.error(error);

	assert!(
		inner_1_teardown_was_called.load(Ordering::Relaxed),
		"inner 1 teardown was not called"
	);
	assert!(
		inner_2_teardown_was_called.load(Ordering::Relaxed),
		"inner 2 teardown was not called"
	);
	assert!(
		inner_3_teardown_was_called.load(Ordering::Relaxed),
		"inner 3 teardown was not called"
	);

	notification_collector.lock().assert_notifications(
		"merge_all - first observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Error(error),
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}
