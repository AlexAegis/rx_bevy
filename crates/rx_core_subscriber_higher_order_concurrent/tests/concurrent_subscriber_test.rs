use std::{
	num::NonZero,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
};

use rx_core::prelude::*;
use rx_core_subscriber_higher_order_concurrent::ConcurrentSubscriber;
use rx_core_testing::prelude::*;

#[test]
fn should_be_able_to_forward_upstream_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(1).unwrap(),
		);

	let error = "error";
	concurrent_subscriber.error(error);

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_complete_from_an_upstream_completion_if_there_are_no_inner_observables() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(1).unwrap(),
		);

	concurrent_subscriber.complete();

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_not_complete_if_upstream_completes_when_there_are_active_inner_observables() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(1).unwrap(),
		);

	let mut inner_subject = PublishSubject::default();
	concurrent_subscriber.next(inner_subject.clone());
	concurrent_subscriber.complete(); // Must not complete because there's an active or queued inner observable

	notification_collector
		.lock()
		.assert_is_empty("concurrent_subscriber");

	inner_subject.complete();

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_move_to_the_next_observable_in_queue_if_the_previous_one_had_finished() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(1).unwrap(),
		);

	let mut inner_1 = PublishSubject::default();
	let mut inner_2 = PublishSubject::default();
	concurrent_subscriber.next(inner_1.clone());
	concurrent_subscriber.next(inner_2.clone());
	concurrent_subscriber.complete(); // Must not complete because there's an active or queued inner observable

	notification_collector
		.lock()
		.assert_is_empty("concurrent_subscriber");

	inner_1.next(1);
	inner_1.complete();
	inner_2.next(2);
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_move_to_the_next_observable_in_queue_if_the_previous_one_had_unsubscribed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(1).unwrap(),
		);

	let mut inner_1 = PublishSubject::default();
	let mut inner_2 = PublishSubject::default();
	concurrent_subscriber.next(inner_1.clone());
	concurrent_subscriber.next(inner_2.clone());
	concurrent_subscriber.complete(); // Must not complete because there's an active or queued inner observable

	notification_collector
		.lock()
		.assert_is_empty("concurrent_subscriber");

	inner_1.next(1);
	inner_1.unsubscribe();
	inner_2.next(2);
	inner_2.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_move_to_the_next_observable_in_queue_if_the_the_concurrency_limit_allows_it() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<PublishSubject<usize, &'static str>, _>::new(
			destination.upgrade(),
			NonZero::new(2).unwrap(),
		);

	let mut inner_1 = PublishSubject::default();
	let mut inner_2 = PublishSubject::default();
	let mut inner_3 = PublishSubject::default();
	concurrent_subscriber.next(inner_1.clone());
	concurrent_subscriber.next(inner_2.clone());
	concurrent_subscriber.next(inner_3.clone());
	concurrent_subscriber.complete(); // Must not complete because there's an active or queued inner observable

	notification_collector
		.lock()
		.assert_is_empty("concurrent_subscriber");

	inner_1.next(1);
	inner_2.next(2);
	inner_1.unsubscribe();
	inner_3.next(3);
	inner_3.unsubscribe();
	inner_2.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_execute_upstream_teardown_on_unsubscribe() {
	let destination = MockObserver::<usize>::default();
	let _notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber = ConcurrentSubscriber::<PublishSubject<usize>, _>::new(
		destination.upgrade(),
		NonZero::new(1).unwrap(),
	);

	let teardown_was_called = Arc::new(AtomicBool::new(false));
	let teardown_was_called_clone = teardown_was_called.clone();
	concurrent_subscriber.add_teardown(Teardown::new(move || {
		teardown_was_called_clone.store(true, Ordering::Relaxed)
	}));

	concurrent_subscriber.unsubscribe();

	assert!(teardown_was_called.load(Ordering::Relaxed));
}

#[test]
fn should_immediately_execute_teardowns_for_a_closed_subscriber() {
	let destination = MockObserver::<usize>::default();
	let _notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber = ConcurrentSubscriber::<PublishSubject<usize>, _>::new(
		destination.upgrade(),
		NonZero::new(1).unwrap(),
	);

	concurrent_subscriber.unsubscribe();

	let teardown_was_called = Arc::new(AtomicBool::new(false));
	let teardown_was_called_clone = teardown_was_called.clone();
	concurrent_subscriber.add_teardown(Teardown::new(move || {
		teardown_was_called_clone.store(true, Ordering::Relaxed)
	}));

	assert!(teardown_was_called.load(Ordering::Relaxed));
}

#[test]
fn should_execute_inner_subscriptions_teardowns() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut concurrent_subscriber =
		ConcurrentSubscriber::<_, _>::new(destination.upgrade(), NonZero::new(1).unwrap());

	let mut subject = PublishSubject::default();
	let teardown_was_called = Arc::new(AtomicBool::new(false));
	let teardown_was_called_clone = teardown_was_called.clone();
	concurrent_subscriber.next(
		subject
			.clone()
			.finalize(move || teardown_was_called_clone.store(true, Ordering::Relaxed)),
	);

	notification_collector
		.lock()
		.assert_is_empty("concurrent_subscriber");

	assert!(!teardown_was_called.load(Ordering::Relaxed));
	subject.complete();
	concurrent_subscriber.unsubscribe();
	assert!(teardown_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"concurrent_subscriber",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}
