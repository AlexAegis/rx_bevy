use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_retry_on_immediate_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let mut retried = concat((
		(0..=2)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		throw(error).map(Never::map_into::<usize>()),
	))
	.retry(1);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_retry_on_later_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let error = "error";
	let mut retried = source
		.clone()
		.on_next(|next, destination| {
			if *next > 10 {
				destination.error(error);
				false
			} else {
				true
			}
		})
		.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	source.next(1);
	source.next(99); // First retry!
	source.next(2);
	source.next(99); // Second retry!
	source.next(3);
	source.next(99); // Error will go through!
	source.next(4);

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_retry_on_mixed_immediate_and_later_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let error = "error";
	let mut retried = concat((
		(0..=1)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		source.clone().on_next(|next, destination| {
			if *next > 10 {
				destination.error(error);
				false
			} else {
				true
			}
		}),
	))
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	source.next(2);
	source.next(99); // First retry!
	source.next(3);
	source.next(4);
	source.next(99); // Second retry!
	source.next(5);
	source.next(99); // Error will go through!
	source.next(6);

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<Never, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let mut retried = throw(error).retry(100);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut retried = (0..=2).into_observable().retry(1);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_immediately_unsubscribed() {
	let destination = MockObserver::<Never>::default();
	let notification_collector = destination.get_notification_collector();

	let mut retried = never().retry(1);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - never");
	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"retry - never",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_retry_on_later_replayable_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = ReplaySubject::<1, usize, &'static str>::default();

	let mut retried = source.clone().retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	let error = "error";

	source.next(1);
	source.error(error);

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_be_able_to_immediately_complete_if_an_immediate_error_was_retried() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let was_retried = Arc::new(AtomicBool::new(false));
	let was_retried_clone = was_retried.clone();

	let error = "error";
	let mut i = 0;
	let mut retried = deferred_observable(move || {
		let observable = if i % 2 == 0 {
			was_retried_clone.store(true, Ordering::Relaxed);
			throw(error).map(Never::map_into::<usize>()).erase()
		} else {
			empty()
				.map(Never::map_into::<usize>())
				.map_error(Never::map_into::<&'static str>())
				.erase()
		};
		i += 1;
		observable
	})
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - deferred");

	notification_collector.lock().assert_notifications(
		"retry - deferred",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(was_retried.load(Ordering::Relaxed));
	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_be_able_to_later_complete_if_an_immediate_error_was_retried() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let was_retried = Arc::new(AtomicBool::new(false));
	let was_retried_clone = was_retried.clone();

	let mut second_source = PublishSubject::<usize, &'static str>::default();
	let second_source_clone = second_source.clone();
	let error = "error";
	let mut i = 0;
	let mut retried = deferred_observable(move || {
		let observable = if i % 2 == 0 {
			was_retried_clone.store(true, Ordering::Relaxed);
			throw(error).map(Never::map_into::<usize>()).erase()
		} else {
			second_source_clone.clone().erase()
		};
		i += 1;
		observable
	})
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - deferred");

	second_source.next(1);
	second_source.complete();

	notification_collector.lock().assert_notifications(
		"retry - deferred",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(was_retried.load(Ordering::Relaxed));
	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_be_able_to_later_error_if_an_immediate_error_was_retried() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let was_retried = Arc::new(AtomicBool::new(false));
	let was_retried_clone = was_retried.clone();

	let mut second_source = PublishSubject::<usize, &'static str>::default();
	let second_source_clone = second_source.clone();
	let error = "error";
	let mut i = 0;
	let mut retried = deferred_observable(move || {
		let observable = if i % 2 == 0 {
			was_retried_clone.store(true, Ordering::Relaxed);
			throw(error).map(Never::map_into::<usize>()).erase()
		} else {
			second_source_clone.clone().erase()
		};
		i += 1;
		observable
	})
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - deferred");

	second_source.next(1);
	second_source.error(error);

	notification_collector.lock().assert_notifications(
		"retry - deferred",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(was_retried.load(Ordering::Relaxed));
	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_be_able_to_later_unsubscribe_if_an_immediate_error_was_retried() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let was_retried = Arc::new(AtomicBool::new(false));
	let was_retried_clone = was_retried.clone();

	let mut second_source = PublishSubject::<usize, &'static str>::default();
	let second_source_clone = second_source.clone();
	let error = "error";
	let mut i = 0;
	let mut retried = deferred_observable(move || {
		let observable = if i % 2 == 0 {
			was_retried_clone.store(true, Ordering::Relaxed);
			throw(error).map(Never::map_into::<usize>()).erase()
		} else {
			second_source_clone.clone().erase()
		};
		i += 1;
		observable
	})
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - deferred");

	second_source.next(1);
	second_source.unsubscribe();

	notification_collector.lock().assert_notifications(
		"retry - deferred",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(was_retried.load(Ordering::Relaxed));
	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_be_able_to_immediately_unsubscribe_if_an_immediate_error_was_retried() {
	let destination = MockObserver::<Never, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let was_retried = Arc::new(AtomicBool::new(false));
	let was_retried_clone = was_retried.clone();

	let error = "error";
	let mut i = 0;
	let mut retried = deferred_observable(move || {
		let observable = if i % 2 == 0 {
			was_retried_clone.store(true, Ordering::Relaxed);
			throw(error).erase()
		} else {
			closed()
				.map_error(Never::map_into::<&'static str>())
				.erase()
		};
		i += 1;
		observable
	})
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - deferred");

	notification_collector.lock().assert_notifications(
		"retry - deferred",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	assert!(was_retried.load(Ordering::Relaxed));
	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}
