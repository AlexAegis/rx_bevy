use rx_core::prelude::*;
use rx_core_subscriber_higher_order_switch::SwitchSubscriber;
use rx_core_testing::prelude::*;

#[test]
fn should_only_allow_new_inner_observables_once_the_current_one_finished() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut switch_subscriber =
		SwitchSubscriber::<PublishSubject<usize, &'static str>, _>::new(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();

	// TODO: InnerObservable on higher order operators must be erased!! Only the output type have to be matched

	// use std::sync::{
	// 	Arc,
	// 	atomic::{AtomicBool, Ordering},
	// };
	// let inner_1_unsubscribed = Arc::new(AtomicBool::new(false));
	// let inner_1_unsubscribed_clone = inner_1_unsubscribed.clone();
	//
	// switch_subscriber.next(
	// 	inner_1
	// 		.clone()
	// 		.finalize(move || inner_1_unsubscribed.store(true, Ordering::Relaxed)),
	// );

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
