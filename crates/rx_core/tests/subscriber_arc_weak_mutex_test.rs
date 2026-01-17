use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

mod before_dropped {
	use super::*;

	#[test]
	fn forwards_next_before_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		weak.next(1);

		notifications.lock().assert_notifications(
			"arc_weak_mutex_next",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);
	}

	#[test]
	fn forwards_error_before_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		weak.error(MockError);

		notifications.lock().assert_notifications(
			"arc_weak_mutex_error",
			0,
			[SubscriberNotification::Error(MockError)],
			true,
		);
	}

	#[test]
	fn forwards_complete_before_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		weak.complete();

		notifications.lock().assert_notifications(
			"arc_weak_mutex_complete",
			0,
			[SubscriberNotification::Complete],
			true,
		);
	}

	#[test]
	fn forwards_unsubscribe_before_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		weak.unsubscribe();

		notifications.lock().assert_notifications(
			"arc_weak_mutex_unsubscribe",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn does_not_run_teardowns_immediately() {
		let (teardown, tracker) = Teardown::tracked("arc_weak_mutex_teardown_while_alive");
		let strong = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));
		let mut weak = Arc::downgrade(&strong);

		weak.add_teardown(teardown);

		tracker.assert_yet_to_be_torn_down();
	}

	#[test]
	fn runs_teardowns_after_drop_and_closes() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let (teardown, tracker) = Teardown::tracked("arc_weak_mutex_teardown_after_drop");

		let weak = {
			let strong = Arc::new(Mutex::new(destination));
			let mut weak = Arc::downgrade(&strong);

			weak.add_teardown(teardown);
			weak.next(1);
			weak.next(2);

			weak
		};

		tracker.assert_was_torn_down();
		assert!(weak.is_closed());

		notifications.lock().assert_notifications(
			"arc_weak_mutex_after_drop",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod when_poisoned {
	use super::*;

	#[test]
	fn unsubscribes_when_mutex_is_poisoned_on_next() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		{
			let poisoned = strong.clone();
			let _ = std::panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		weak.next(99);

		notifications.lock().assert_notifications(
			"arc_weak_mutex_poison_next",
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
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		{
			let poisoned = strong.clone();
			let _ = std::panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		weak.error(MockError);

		notifications.lock().assert_notifications(
			"arc_weak_mutex_poison_error",
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
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		{
			let poisoned = strong.clone();
			let _ = std::panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		weak.complete();

		notifications.lock().assert_notifications(
			"arc_weak_mutex_poison_complete",
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
		let strong = Arc::new(Mutex::new(destination));
		let mut weak = Arc::downgrade(&strong);

		{
			let poisoned = strong.clone();
			let _ = std::panic::catch_unwind(|| {
				let mut guard = poisoned.lock().unwrap();
				guard.next(1);
				mute_panic(|| panic!("poison"));
			});
		}

		weak.unsubscribe();

		notifications.lock().assert_notifications(
			"arc_weak_mutex_poison_unsubscribe",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod after_dropped {
	use super::*;

	#[test]
	fn ignores_next_after_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut weak = {
			let strong = Arc::new(Mutex::new(destination));
			Arc::downgrade(&strong)
		};

		weak.next(1);

		assert!(weak.is_closed());
		notifications.lock().assert_notifications(
			"arc_weak_mutex_after_drop_next",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn ignores_error_after_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut weak = {
			let strong = Arc::new(Mutex::new(destination));
			Arc::downgrade(&strong)
		};

		weak.error(MockError);

		assert!(weak.is_closed());
		notifications.lock().assert_notifications(
			"arc_weak_mutex_after_drop_error",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn ignores_complete_after_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut weak = {
			let strong = Arc::new(Mutex::new(destination));
			Arc::downgrade(&strong)
		};

		weak.complete();

		assert!(weak.is_closed());
		notifications.lock().assert_notifications(
			"arc_weak_mutex_after_drop_complete",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn ignores_unsubscribe_after_drop() {
		let destination = MockObserver::<usize, MockError>::default();
		let notifications = destination.get_notification_collector();
		let mut weak = {
			let strong = Arc::new(Mutex::new(destination));
			Arc::downgrade(&strong)
		};

		weak.unsubscribe();

		assert!(weak.is_closed());
		notifications.lock().assert_notifications(
			"arc_weak_mutex_after_drop_unsubscribe",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}

	#[test]
	fn executes_teardown_when_target_is_missing() {
		let flag = Arc::new(AtomicBool::new(false));
		let mut weak = {
			let strong = Arc::new(Mutex::new(MockObserver::<usize, MockError>::default()));
			Arc::downgrade(&strong)
		};

		let flag_clone = flag.clone();
		weak.add_teardown(Teardown::new(move || {
			flag_clone.store(true, Ordering::Relaxed)
		}));

		assert!(flag.load(Ordering::Relaxed));
	}

	#[test]
	fn returns_closed_subscription_if_target_is_missing() {
		let mut dropped_weak_observable = {
			let strong = Arc::new(Mutex::new(JustObservable::<usize>::default()));
			Arc::downgrade(&strong)
		};

		let destination = MockObserver::<usize>::default();
		let notifications = destination.get_notification_collector();

		let subscription = dropped_weak_observable.subscribe(destination);

		assert!(subscription.is_closed());
		notifications.lock().assert_notifications(
			"arc_weak_mutex_subscribe_missing",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
	}
}
