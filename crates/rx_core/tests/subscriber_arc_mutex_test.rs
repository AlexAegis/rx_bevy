use std::{
	panic,
	sync::{Arc, Mutex},
};

use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn forwards_next_through_mutex() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(Mutex::new(destination));

	subscriber.next(1);

	notifications.lock().assert_notifications(
		"arc_mutex",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);
}

#[test]
fn forwards_error_through_mutex() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(Mutex::new(destination));

	subscriber.error(MockError);

	notifications.lock().assert_notifications(
		"arc_mutex",
		0,
		[SubscriberNotification::Error(MockError)],
		true,
	);
}

#[test]
fn forwards_complete_through_mutex() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(Mutex::new(destination));

	subscriber.complete();

	notifications.lock().assert_notifications(
		"arc_mutex",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn forwards_unsubscribe_through_mutex() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(Mutex::new(destination));

	subscriber.unsubscribe();

	notifications.lock().assert_notifications(
		"arc_mutex",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn forwards_subscription_calls() {
	let subject = Arc::new(Mutex::new(JustObservable::<usize>::new(1)));
	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();

	let subscription = subject.clone().subscribe(destination);

	notifications.lock().assert_notifications(
		"arc_mutex",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

mod when_poisoned {
	use super::*;

	#[test]
	fn unsubscribes_when_mutex_is_poisoned_on_next() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(Mutex::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.next(99);

		notifications.lock().assert_notifications(
			"arc_mutex",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_mutex_is_poisoned_on_error() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(Mutex::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.error(MockError);

		notifications.lock().assert_notifications(
			"arc_mutex",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_mutex_is_poisoned_on_complete() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(Mutex::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.complete();

		notifications.lock().assert_notifications(
			"arc_mutex",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_mutex_is_poisoned_on_unsubscribe() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(Mutex::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.unsubscribe();

		notifications.lock().assert_notifications(
			"arc_mutex",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod shared_destination {
	use super::*;

	#[test]
	fn access_invokes_closure() {
		let reported_closed = Arc::new(AtomicBool::new(false));
		let arc_destination = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));

		let mut shared = arc_destination.clone();
		shared.access(|destination| {
			reported_closed.store(destination.is_closed(), Ordering::Relaxed);
		});

		assert_eq!(reported_closed.load(Ordering::Relaxed), shared.is_closed());
	}

	#[test]
	fn access_mut_invokes_closure() {
		let arc_destination = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));

		let mut shared = arc_destination.clone();
		shared.access_mut(|destination| {
			destination.unsubscribe();
		});

		assert!(arc_destination.lock().unwrap().is_closed());
	}

	#[test]
	fn add_teardown_executes_on_unsubscribe() {
		let arc_destination = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));
		let mut shared = arc_destination.clone();
		let (teardown, tracker) = Teardown::tracked("arc_mutex");

		shared.add_teardown(teardown);
		tracker.assert_yet_to_be_torn_down();

		shared.unsubscribe();

		tracker.assert_was_torn_down();
		assert!(shared.is_closed());
	}

	#[test]
	fn add_teardown_executes_when_poisoned() {
		let arc_destination = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));
		{
			let poisoned = arc_destination.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut shared = arc_destination.clone();
		let (teardown, tracker) = Teardown::tracked("arc_mutex");

		shared.add_teardown(teardown);

		tracker.assert_was_torn_down();
		assert!(shared.is_closed());
	}
}
