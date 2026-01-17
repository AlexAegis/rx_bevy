use std::{
	panic,
	sync::{Arc, RwLock},
};

use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[test]
fn forwards_next_through_rw_lock() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(RwLock::new(destination));

	subscriber.next(1);

	notifications.lock().assert_notifications(
		"arc_rw_lock_next",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);
}

#[test]
fn forwards_error_through_rw_lock() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(RwLock::new(destination));

	subscriber.error(MockError);

	notifications.lock().assert_notifications(
		"arc_rw_lock_error",
		0,
		[SubscriberNotification::Error(MockError)],
		true,
	);
}

#[test]
fn forwards_complete_through_rw_lock() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(RwLock::new(destination));

	subscriber.complete();

	notifications.lock().assert_notifications(
		"arc_rw_lock_complete",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn forwards_unsubscribe_through_rw_lock() {
	let destination = MockObserver::<usize, MockError>::default();
	let notifications = destination.get_notification_collector();
	let mut subscriber = Arc::new(RwLock::new(destination));

	subscriber.unsubscribe();

	notifications.lock().assert_notifications(
		"arc_rw_lock_unsubscribe",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn forwards_subscription_calls() {
	let subject = Arc::new(RwLock::new(JustObservable::<usize>::default()));
	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();

	let mut shared_subject = subject.clone();
	let subscription = shared_subject.subscribe(destination);

	notifications.lock().assert_notifications(
		"arc_rw_lock_subscribe",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

mod when_poisoned {
	use super::*;

	#[test]
	fn unsubscribes_when_rw_lock_is_poisoned_on_next() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(RwLock::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.write().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.next(99);

		notifications.lock().assert_notifications(
			"arc_rw_lock_poison",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_rw_lock_is_poisoned_on_error() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(RwLock::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.write().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.error(MockError);

		notifications.lock().assert_notifications(
			"arc_rw_lock_poison_error",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_rw_lock_is_poisoned_on_complete() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(RwLock::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.write().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.complete();

		notifications.lock().assert_notifications(
			"arc_rw_lock_poison_complete",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn unsubscribes_when_rw_lock_is_poisoned_on_unsubscribe() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let subscriber = Arc::new(RwLock::new(destination));

		{
			let poisoned = subscriber.clone();
			let _ = panic::catch_unwind(|| {
				let mut guard = poisoned.write().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		let mut poisoned = subscriber.clone();
		poisoned.unsubscribe();

		notifications.lock().assert_notifications(
			"arc_rw_lock_poison_unsubscribe",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}
