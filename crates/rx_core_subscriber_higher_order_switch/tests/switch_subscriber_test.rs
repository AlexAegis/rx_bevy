use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};

use rx_core::prelude::*;
use rx_core_subscriber_higher_order_switch::SwitchSubscriber;
use rx_core_testing::prelude::*;

#[test]
fn should_switch_to_new_input_observables_immediately() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();

	switch_subscriber.next(inner_1.clone());

	notification_collector
		.lock()
		.assert_is_empty("switch_subscriber");

	inner_1.next(0);
	inner_2.next(99); // Should be ignored!
	inner_1.next(1);

	inner_2.next(98); // Should be ignored!

	switch_subscriber.next(inner_2.clone()); // It should subscribe to it even when the previous one had not finished yet!

	inner_2.next(2);
	inner_2.next(3);
	switch_subscriber.complete(); // Should not complete it just yet, there is an active inner subscription!
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"switch_subscriber",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_immediately_complete_if_there_are_no_active_subscriptions() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	switch_subscriber.complete();

	notification_collector.lock().assert_notifications(
		"switch_subscriber",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_immediately_error_by_an_inner_error() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();

	switch_subscriber.next(inner_1.clone());
	switch_subscriber.next(inner_2.clone());

	notification_collector
		.lock()
		.assert_is_empty("switch_subscriber");

	let error = "error";
	inner_1.error(error);

	notification_collector
		.lock()
		.assert_is_empty("switch_subscriber - after an already unsubscribed source errored");

	inner_2.error(error);

	notification_collector.lock().assert_notifications(
		"switch_subscriber",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_immediately_error_by_an_upstream_error() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let error = "error";
	switch_subscriber.error(error);

	notification_collector.lock().assert_notifications(
		"switch_subscriber",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_also_run_teardowns_on_unsubscribe() {
	let destination = MockObserver::<usize, &'static str>::default();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let tracked_teardown = switch_subscriber.add_tracked_teardown("exhaust");

	switch_subscriber.unsubscribe();

	tracked_teardown.assert_was_torn_down();

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_also_run_teardowns_immediately_when_added_to_an_already_closed_subscriber() {
	let destination = MockObserver::<usize, &'static str>::default();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	switch_subscriber.unsubscribe();

	let tracked_teardown = switch_subscriber.add_tracked_teardown("exhaust");

	tracked_teardown.assert_was_torn_down();

	assert!(switch_subscriber.is_closed())
}

#[test]
fn should_run_teardowns_of_inner_subscribers_when_switching() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<ErasedObservable<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();

	let inner_1_unsubscribed = Arc::new(AtomicBool::new(false));
	let inner_1_unsubscribed_clone = inner_1_unsubscribed.clone();

	switch_subscriber.next(
		inner_1
			.clone()
			.finalize(move || inner_1_unsubscribed_clone.store(true, Ordering::Relaxed))
			.erase(),
	);

	switch_subscriber.next(inner_1.clone().erase());

	notification_collector
		.lock()
		.assert_is_empty("switch_subscriber");

	inner_1.next(0);
	inner_1.next(1);

	switch_subscriber.next(inner_2.clone().erase()); // It should subscribe to it even when the previous one had not finished yet!

	assert!(inner_1_unsubscribed.load(Ordering::Relaxed));
}

#[test]
fn should_be_able_to_unsubscribe_early_if_downstream_is_closed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber = SwitchSubscriber::<ErasedObservable<usize, &'static str>, _>::new(
		TakeOperator::new(1).operator_subscribe(destination),
	);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();

	let inner_1_unsubscribed = Arc::new(AtomicBool::new(false));
	let inner_1_unsubscribed_clone = inner_1_unsubscribed.clone();

	switch_subscriber.next(
		inner_1
			.clone()
			.finalize(move || inner_1_unsubscribed_clone.store(true, Ordering::Relaxed))
			.erase(),
	);

	inner_1.next(0);

	switch_subscriber.unsubscribe();

	inner_1.next(1);
	notification_collector.print();

	notification_collector.lock().assert_notifications(
		"switch_subscriber",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(inner_1_unsubscribed.load(Ordering::Relaxed));
}
