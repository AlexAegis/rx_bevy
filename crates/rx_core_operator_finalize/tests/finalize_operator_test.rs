use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_call_the_finalizer_on_unsubscribe() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();

	let finalizer_was_called = Arc::new(AtomicBool::new(false));
	let finalizer_was_called_clone = finalizer_was_called.clone();
	let mut subscription = source
		.clone()
		.finalize(move || finalizer_was_called_clone.store(true, Ordering::Relaxed))
		.subscribe(destination);

	assert!(!subscription.is_closed());
	subscription.unsubscribe();
	assert!(subscription.is_closed());

	assert!(finalizer_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"finalize",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_call_the_finalizer_when_unsubscribed_by_another_operator() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let finalizer_was_called = Arc::new(AtomicBool::new(false));
	let finalizer_was_called_clone = finalizer_was_called.clone();
	let subscription = source
		.clone()
		.take(1)
		.finalize(move || finalizer_was_called_clone.store(true, Ordering::Relaxed))
		.subscribe(destination);

	assert!(!subscription.is_closed());
	source.next(0);
	assert!(subscription.is_closed());

	assert!(finalizer_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"finalize",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let finalizer_was_called = Arc::new(AtomicBool::new(false));
	let finalizer_was_called_clone = finalizer_was_called.clone();
	let subscription = source
		.clone()
		.finalize(move || finalizer_was_called_clone.store(true, Ordering::Relaxed))
		.subscribe(destination);

	let error = "error";
	source.error(error);

	assert!(subscription.is_closed());
	assert!(finalizer_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"finalize",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let finalizer_was_called = Arc::new(AtomicBool::new(false));
	let finalizer_was_called_clone = finalizer_was_called.clone();
	let subscription = source
		.clone()
		.finalize(move || finalizer_was_called_clone.store(true, Ordering::Relaxed))
		.subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());
	assert!(finalizer_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"finalize",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let finalizer_was_called = Arc::new(AtomicBool::new(false));
	let finalizer_was_called_clone = finalizer_was_called.clone();
	let composed = compose_operator::<usize, &'static str>()
		.finalize(move || finalizer_was_called_clone.store(true, Ordering::Relaxed));

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());
	assert!(finalizer_was_called.load(Ordering::Relaxed));

	notification_collector.lock().assert_notifications(
		"finalize",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let finalizer_called = Arc::new(AtomicBool::new(false));
		let finalizer_called_clone = finalizer_called.clone();
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("finalize");
		let observable = harness
			.create_harness_observable()
			.finalize(move || finalizer_called_clone.store(true, Ordering::Relaxed));
		harness.subscribe_to(observable);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
		assert!(finalizer_called.load(Ordering::Relaxed));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let finalizer_called = Arc::new(AtomicBool::new(false));
		let finalizer_called_clone = finalizer_called.clone();
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("finalize");
		let observable = harness
			.create_harness_observable()
			.finalize(move || finalizer_called_clone.store(true, Ordering::Relaxed));
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
		assert!(finalizer_called.load(Ordering::Relaxed));
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let finalizer_called = Arc::new(AtomicBool::new(false));
		let finalizer_called_clone = finalizer_called.clone();
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("finalize");
		let observable = harness
			.create_harness_observable()
			.finalize(move || finalizer_called_clone.store(true, Ordering::Relaxed));
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
		assert!(finalizer_called.load(Ordering::Relaxed));
	}
}
