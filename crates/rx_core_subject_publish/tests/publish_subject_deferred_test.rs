use std::sync::{
	Arc, Mutex,
	atomic::{AtomicUsize, Ordering},
};

use rx_core::prelude::*;
use rx_core_subject_publish::internal::MulticastSubscription;
use rx_core_testing::prelude::*;

#[test]
fn should_be_able_to_tear_itself_down() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize>::default();

	let mut finalize_subject = subject.clone();
	let subscription = subject
		.clone()
		.take(1)
		.finalize(move || {
			finalize_subject.unsubscribe();
		})
		.subscribe(destination);

	assert!(
		notification_collector.lock().is_empty(),
		"nothing should've happened yet"
	);

	subject.next(0); // Should trigger an unsubscribe due to finalize

	notification_collector.lock().assert_notifications(
		"Self tearing subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	assert!(subject.is_closed());
}

#[test]
fn should_be_able_to_subscribe_from_teardown() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let notification_collector_2 = NotificationCollector::default();

	let mut subject = PublishSubject::<usize>::default();

	let mut finalize_subject = subject.clone();

	let final_subscription = Arc::new(Mutex::new(None::<MulticastSubscription<usize>>));
	let final_subscription_clone = final_subscription.clone();
	let final_notification_collector_2 = notification_collector_2.clone();
	let subscription = subject
		.clone()
		.take(1)
		.finalize(move || {
			let subscription =
				finalize_subject.subscribe(MockObserver::new(final_notification_collector_2));

			final_subscription_clone
				.lock_ignore_poison()
				.replace(subscription);
		})
		.subscribe(destination_1);

	assert!(
		notification_collector_1.lock().is_empty(),
		"nothing should've happened yet"
	);

	subject.next(0); // Should trigger a new subscription

	assert!(
		subscription.is_closed(),
		"should be closed because of take(1)"
	);

	subject.next(1);

	subject.complete();

	notification_collector_1.lock().assert_notifications(
		"Notification Collector 1",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe, // Because of `take`
		],
		true,
	);

	notification_collector_2.lock().assert_notifications(
		"Notification Collector 2",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_subscribe_and_next_to_the_new_subscription_from_teardown() {
	let destination_1 = MockObserver::default();
	let notification_collector_outer = destination_1.get_notification_collector();

	let multi_round_notification_collector = MultiRoundNotificationCollector::default();

	let mut subject = PublishSubject::<usize>::default();
	let final_subscription = SubscriptionHandle::default();
	let arc_round = Arc::new(AtomicUsize::default());

	let mut clojures_subject = subject.clone();
	let mut clojures_subscription_clone = final_subscription.clone();
	let clojure_multi_round_notification_collector = multi_round_notification_collector.clone();
	let clojure_round = arc_round.clone();

	let subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			let subscription = clojures_subject.subscribe(MockObserver::new(
				clojure_multi_round_notification_collector
					.lock()
					.get_round(clojure_round.load(Ordering::Relaxed)),
			));
			// Self feeding subject, a recipe for disaster!
			clojures_subject.next(next + 1);
			clojure_round.fetch_add(1, Ordering::Relaxed);
			clojures_subscription_clone.add(subscription);
		})
		.subscribe(destination_1);

	assert!(
		notification_collector_outer.lock().is_empty(),
		"nothing should've happened yet"
	);

	subject.next(0); // Should trigger a new subscription

	assert!(
		subscription.is_closed(),
		"should be closed because of take(1)"
	);

	subject.complete();

	notification_collector_outer.lock().assert_notifications(
		"Notification Collector outer",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe, // Because of take
		],
		true,
	);

	multi_round_notification_collector
		.lock()
		.get_round(0)
		.lock()
		.assert_notifications(
			"Notification Collector Round 0",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

	multi_round_notification_collector
		.lock()
		.get_round(1)
		.lock()
		.assert_notifications(
			"Notification Collector Round 1",
			0,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
}

#[test]
#[should_panic]
fn should_be_able_to_detect_a_simple_infinite_loop_and_panic() {
	let destination_1 = MockObserver::default();

	let mut subject = PublishSubject::<usize>::default();

	let mut subject_level_1 = subject.clone();

	let _subscription = subject
		.clone()
		.tap_next(move |next| {
			subject_level_1.next(next + 1);
		})
		.subscribe(destination_1);

	mute_panic(|| subject.next(0)); // Infinite loop!
}

#[test]
fn should_be_able_to_defer_and_stop_at_an_error() {
	let destination_1 = MockObserver::default();
	let notification_collector = destination_1.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subject_clone = subject.clone();

	let subscription = subject
		.clone()
		.tap_next(move |next| {
			subject_clone.next(next + 1);
			subject_clone.error("error");
		})
		.subscribe(destination_1);

	subject.next(0);

	assert!(subject.is_errored());
	assert!(subject.is_closed());
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error("error"),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_defer_and_stop_at_a_completion() {
	let destination_1 = MockObserver::default();
	let notification_collector = destination_1.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subject_clone = subject.clone();

	let subscription = subject
		.clone()
		.tap_next(move |next| {
			subject_clone.next(next + 1);
			subject_clone.complete();
		})
		.subscribe(destination_1);

	subject.next(0);

	assert!(!subject.is_errored());
	assert!(subject.is_closed());
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_defer_a_pre_closed_subscription_to_itself_without_adding_it_to_the_subscribers()
 {
	let destination_outer = MockObserver::default();
	let notification_collector_outer = destination_outer.get_notification_collector();

	let notification_collector_inner = NotificationCollector::default();
	let notification_collector_inner_clone = notification_collector_inner.clone();
	let mut subject = PublishSubject::<usize, &'static str>::default();

	let subject_clone = subject.clone();

	let subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			if *next == 0 {
				let mut subscription =
					subject_clone
						.clone()
						.start_with(|| 10)
						.subscribe(MockObserver::new(
							notification_collector_inner_clone.clone(),
						));
				subscription.unsubscribe();
			}
		})
		.subscribe(destination_outer);

	subject.next(0);
	subject.next(1);

	assert!(!subject.is_errored());
	assert!(subscription.is_closed());

	notification_collector_outer.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_inner.lock().assert_notifications(
		"Inner",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_defer_an_active_subscription_to_itself_without_adding_it_to_the_subscribers_if_the_subject_itself_was_deferred_to_be_unsubscribed_before_the_subscription()
 {
	let destination_outer = MockObserver::default();
	let notification_collector_outer = destination_outer.get_notification_collector();

	let notification_collector_inner = NotificationCollector::default();
	let notification_collector_inner_clone = notification_collector_inner.clone();
	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subject_clone = subject.clone();

	let subscription_handle = SubscriptionHandle::default();
	let mut subscription_handle_clone = subscription_handle.clone();

	let subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			if *next == 0 {
				subject_clone.unsubscribe();

				let subscription =
					subject_clone
						.clone()
						.start_with(|| 10)
						.subscribe(MockObserver::new(
							notification_collector_inner_clone.clone(),
						));
				subscription_handle_clone.add(subscription); // Dropping it would unsubscribe!
			}
		})
		.subscribe(destination_outer);

	subject.next(0); // Unsubscribes in the tap!
	subject.next(1);

	assert!(!subject.is_errored());
	assert!(subscription.is_closed());

	notification_collector_outer.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_inner.lock().assert_notifications(
		"Inner",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_defer_an_active_subscription_to_itself_without_adding_it_to_the_subscribers_if_the_subject_itself_was_deferred_to_be_errored_before_the_subscription()
 {
	let destination_outer = MockObserver::default();
	let notification_collector_outer = destination_outer.get_notification_collector();

	let notification_collector_inner = NotificationCollector::default();
	let notification_collector_inner_clone = notification_collector_inner.clone();
	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subject_clone = subject.clone();

	let subscription_handle = SubscriptionHandle::default();
	let mut subscription_handle_clone = subscription_handle.clone();

	let subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			if *next == 0 {
				subject_clone.error("error");

				let subscription =
					subject_clone
						.clone()
						.start_with(|| 10)
						.subscribe(MockObserver::new(
							notification_collector_inner_clone.clone(),
						));
				subscription_handle_clone.add(subscription); // Dropping it would unsubscribe!
			}
		})
		.subscribe(destination_outer);

	subject.next(0); // Unsubscribes in the tap!
	subject.next(1);

	assert!(subject.is_errored());
	assert!(subject.is_closed());
	assert!(subscription.is_closed());

	notification_collector_outer.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Error("error"),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_inner.lock().assert_notifications(
		"Inner",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Error("error"),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_defer_an_active_subscription_to_itself_without_adding_it_to_the_subscribers_if_the_subject_itself_was_deferred_to_be_completed_before_the_subscription()
 {
	let destination_outer = MockObserver::default();
	let notification_collector_outer = destination_outer.get_notification_collector();

	let notification_collector_inner = NotificationCollector::default();
	let notification_collector_inner_clone = notification_collector_inner.clone();
	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subject_clone = subject.clone();

	let subscription_handle = SubscriptionHandle::default();
	let mut subscription_handle_clone = subscription_handle.clone();

	let subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			if *next == 0 {
				subject_clone.complete();

				let subscription =
					subject_clone
						.clone()
						.start_with(|| 10)
						.subscribe(MockObserver::new(
							notification_collector_inner_clone.clone(),
						));
				subscription_handle_clone.add(subscription); // Dropping it would unsubscribe!
			}
		})
		.subscribe(destination_outer);

	subject.next(0); // Unsubscribes in the tap!
	subject.next(1);

	assert!(!subject.is_errored());
	assert!(subject.is_closed());
	assert!(subscription.is_closed());

	notification_collector_outer.lock().assert_notifications(
		"subject",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_inner.lock().assert_notifications(
		"Inner",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

/// Do not ever even attempt to write something like this.
/// Just because there's a test for it, it does not mean it's a good idea.
/// This is the absolute opposite of a good idea.
#[test]
fn should_be_able_to_handle_a_lot_of_nested_deferred_events() {
	let destination_1 = MockObserver::default();

	let mut subject = PublishSubject::<String>::default();

	let final_subscription = SubscriptionHandle::default();

	let notification_collector_level_1 = MultiRoundNotificationCollector::default();
	let notification_collector_level_2 = MultiRoundNotificationCollector::default();
	let notification_collector_level_3 = MultiRoundNotificationCollector::default();
	let arc_round_level_1 = Arc::new(AtomicUsize::default());
	let arc_round_level_2 = Arc::new(AtomicUsize::default());
	let arc_round_level_3 = Arc::new(AtomicUsize::default());

	let mut subject_level_1 = subject.clone();
	let subject_level_2 = subject.clone();
	let subject_level_3 = subject.clone();
	let collector_level_1 = notification_collector_level_1.clone();
	let collector_level_2 = notification_collector_level_2.clone();
	let collector_level_3 = notification_collector_level_3.clone();
	let mut subscription_handle_level_1 = final_subscription.clone();
	let subscription_handle_level_2 = final_subscription.clone();
	let subscription_handle_level_3 = final_subscription.clone();
	let round_level_1 = arc_round_level_1.clone();
	let round_level_2 = arc_round_level_2.clone();
	let round_level_3 = arc_round_level_3.clone();

	let take_count = 3;

	let mut subscription = subject
		.clone()
		.take(take_count)
		.tap_next(move |next| {
			let mut subject_level_2 = subject_level_2.clone();
			let collector_level_2 = collector_level_2.clone();
			let mut subscription_handle_level_2 = subscription_handle_level_2.clone();
			let round_level_2 = round_level_2.clone();

			let subject_level_3 = subject_level_3.clone();
			let collector_level_3 = collector_level_3.clone();
			let subscription_handle_level_3 = subscription_handle_level_3.clone();
			let round_level_3 = round_level_3.clone();

			let subscription = subject_level_1
				.clone()
				.take(take_count)
				.tap_next(move |next| {
					let mut subject_level_3 = subject_level_3.clone();
					let collector_level_3 = collector_level_3.clone();
					let mut subscription_handle_level_3 = subscription_handle_level_3.clone();
					let round_level_3 = round_level_3.clone();

					let subscription = subject_level_2
						.clone()
						.take(take_count)
						.tap_next(move |next| {
							let subscription = subject_level_3.clone().take(take_count).subscribe(
								MockObserver::new(
									collector_level_3
										.lock()
										.get_round(round_level_3.load(Ordering::Relaxed)),
								),
							);
							subject_level_3
								.next(format!("{next} {}", round_level_3.load(Ordering::Relaxed)));
							subscription_handle_level_3.add(subscription);

							round_level_3.fetch_add(1, Ordering::Relaxed);
						})
						.subscribe(MockObserver::new(
							collector_level_2
								.lock()
								.get_round(round_level_2.load(Ordering::Relaxed)),
						));
					subject_level_2
						.next(format!("{next} {}", round_level_2.load(Ordering::Relaxed)));
					subscription_handle_level_2.add(subscription);

					round_level_2.fetch_add(1, Ordering::Relaxed);
				})
				.subscribe(MockObserver::new(
					collector_level_1
						.lock()
						.get_round(round_level_1.load(Ordering::Relaxed)),
				));
			subject_level_1.next(format!("{next} {}", round_level_1.load(Ordering::Relaxed)));
			subscription_handle_level_1.add(subscription);

			round_level_1.fetch_add(1, Ordering::Relaxed);
		})
		.subscribe(destination_1);

	subject.next("START".to_string());

	subject.complete();

	subscription.unsubscribe();

	let _d = dbg!(notification_collector_level_1.lock());
	let _d = dbg!(notification_collector_level_2.lock());
	let _d = dbg!(notification_collector_level_3.lock());
}

/// Don't do this either.
#[test]
fn should_be_able_to_recursively_subscribe_to_itself() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut subject = PublishSubject::<String>::default();

	let final_subscription = SubscriptionHandle::default();

	let notification_collector_level_1 = MultiRoundNotificationCollector::default();
	let arc_round_level_1 = Arc::new(AtomicUsize::default());

	let mut subject_level_1 = subject.clone();
	let collector_level_1 = notification_collector_level_1.clone();
	let mut subscription_handle_level_1 = final_subscription.clone();
	let round_level_1 = arc_round_level_1.clone();

	let mut subscription = subject
		.clone()
		.take(2)
		.tap_next(move |next| {
			let subscription = subject_level_1.clone().take(2).subscribe(MockObserver::new(
				collector_level_1
					.lock()
					.get_round(round_level_1.load(Ordering::Relaxed)),
			));
			subject_level_1.next(format!("{next} {}", round_level_1.load(Ordering::Relaxed)));
			subscription_handle_level_1.add(subscription);

			round_level_1.fetch_add(1, Ordering::Relaxed);
		})
		.subscribe(destination_1);

	subject.next("START".to_string());
	subject.complete();

	notification_collector_1.lock().assert_notifications(
		"OUTER",
		0,
		[
			SubscriberNotification::Next("START".to_string()),
			SubscriberNotification::Next("START 0".to_string()),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	notification_collector_level_1
		.lock()
		.get_round(0)
		.lock()
		.assert_notifications(
			"collector_level_1 round 0",
			0,
			[
				SubscriberNotification::Next("START 0".to_string()),
				SubscriberNotification::Next("START 0 1".to_string()),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

	notification_collector_level_1
		.lock()
		.get_round(1)
		.lock()
		.assert_notifications(
			"collector_level_1 round 0",
			0,
			[
				SubscriberNotification::Next("START 0 1".to_string()),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

	assert!(!notification_collector_level_1.lock().has_round(2));

	subject.complete();

	subscription.unsubscribe();
}
