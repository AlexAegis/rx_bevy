use std::sync::{Arc, Mutex};

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_forward_values_to_multiple_active_listeners_after_completion() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let destination_2 = MockObserver::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let mut subject = PublishSubject::<usize>::default();

	subject.next(0); // There are no listeners so nobody should receive it

	let _s = subject.clone().subscribe(destination_1);

	assert!(
		notification_collector_1.lock().is_empty(),
		"Nothing should've been replayed"
	);

	subject.next(1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(1),
		"destination_1 did not receive the first emission"
	);

	let _s = subject.clone().subscribe(destination_2);

	subject.next(2);
	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Next(2),
		"destination_1 did not receive the second emission"
	);

	assert_eq!(
		notification_collector_2.lock().nth_notification(0),
		&SubscriberNotification::Next(2),
		"destination_2 did not receive the second emission, which is first for this subscription"
	);

	subject.complete();

	notification_collector_1.lock().assert_notifications(
		"destination_1 did not receive the completion signal",
		2,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
	notification_collector_2.lock().assert_notifications(
		"destination_2 did not receive the completion signal",
		1,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

// In rxjs this is a panic. rx_core instead chooses to immediately unsubscribe
// when trying to make an invalid subscription to a subject when it was already
// closed.
#[test]
fn should_immediately_complete_and_unsubscribe_new_subscribers_if_already_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize>::default();

	subject.next(0);
	subject.complete();

	let mut subscription = subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(2),
		"destination received an additional signal after already unsubscribed!"
	);
}

// In rxjs this is a panic. rx_core instead chooses to immediately unsubscribe
// when trying to make an invalid subscription to a subject when it was already
// closed.
#[test]
fn should_immediately_error_new_subscribers_if_errored_before_the_subscription() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let error = "error";
	subject.error(error);
	subject.complete(); // Must have no effect after an error!

	let mut subscription = subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"publish_subject",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(2),
		"destination received an additional signal after already unsubscribed!"
	);
}

#[test]
fn should_not_unsubscribe_existing_subscribers_if_not_manually_unsubscribed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let subject = PublishSubject::<usize>::default();

	let _subscription = subject.clone().subscribe(destination);

	let mut source = PublishSubject::<usize>::default();
	let mut source_subscription = source.subscribe(subject.clone());
	source.next(0);
	source_subscription.unsubscribe();
	assert!(source_subscription.is_closed());
	assert!(!source.is_closed());

	assert!(
		!subject.is_closed(),
		"the subject should not have unsubscribed because a subscription it was a destination for unsubscribed"
	);

	notification_collector.lock().assert_notifications(
		"subject_as_destination",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);
}

#[test]
fn should_unsubscribe_existing_subscribers_if_manually_unsubscribed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subscription = subject.clone().subscribe(destination);

	subject.unsubscribe();

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"destination received an additional signal after already unsubscribed!"
	);
}

#[test]
fn should_immediately_unsubscribe_new_subscribers_if_already_closed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	subject.unsubscribe();

	let mut subscription = subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"destination received an additional signal after already unsubscribed!"
	);
}

#[test]
fn should_immediately_complete_and_unsubscribe_new_subscribers_if_already_completed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();
	subject.complete();

	let mut subscription = subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(
		!notification_collector.lock().nth_notification_exists(2),
		"destination received an additional signal after already unsubscribed!"
	);

	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(2),
		"destination received an additional signal after already unsubscribed!"
	);
}

#[test]
fn should_immediately_error_and_unsubscribe_new_subscribers_if_errored_and_unsubscribed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();
	let error = "error";
	subject.error(error);

	let mut subscription = subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(2),
		"destination received an additional signal after already unsubscribed!"
	);
}

#[test]
fn should_be_able_to_chain_subjects_as_destinations() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source_subject = PublishSubject::<usize, &'static str>::default();
	let mut relay_subject = PublishSubject::<String, &'static str>::default();

	let mut upstream_subscription = source_subject
		.clone()
		.map(|n| format!("foo{}", n))
		.subscribe(relay_subject.clone());

	source_subject.next(1);

	assert!(
		!notification_collector.lock().nth_notification_exists(0),
		"destination received a signal before subscribe!"
	);

	let mut downstream_subscription = relay_subject.subscribe(destination);

	source_subject.next(1);

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Next("foo1".to_string()),
		"destination did not receive the first signal"
	);

	upstream_subscription.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"destination should not have receive an unsubscribe notification because upstream unsubscribed, they should be detached!"
	);

	source_subject.next(2);

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"destination should not have receive a new value because upstream unsubscribed already!"
	);

	// Re-establish upstream
	let mut upstream_subscription_2 = source_subject
		.clone()
		.map(|n| format!("bar{}", n))
		.subscribe(relay_subject.clone());

	source_subject.next(3);

	assert_eq!(
		notification_collector.lock().nth_notification(1),
		&SubscriberNotification::Next("bar3".to_string()),
		"destination did not receive the second signal"
	);

	source_subject.complete();

	assert_eq!(
		notification_collector.lock().nth_notification(2),
		&SubscriberNotification::Complete,
		"destination did not receive the completion signal"
	);

	source_subject.next(99); // Has no effect after completion/error

	downstream_subscription.unsubscribe();

	assert_eq!(
		notification_collector.lock().nth_notification(3),
		&SubscriberNotification::Unsubscribe,
		"destination did not receive the unsubscribe signal"
	);

	upstream_subscription_2.unsubscribe();
}

#[test]
fn should_complete_active_subscribers() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let _subscription = subject.clone().subscribe(destination);

	subject.complete();

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_error_active_subscribers_but_not_unsubscribe_them() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let _subscription = subject.clone().subscribe(destination);

	let error = "error";
	subject.error(error);

	notification_collector.lock().assert_notifications(
		"publish_subject destination",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_closed_after_completion() {
	let mut subject = PublishSubject::<usize, &'static str>::default();
	subject.complete();
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_error() {
	let mut subject = PublishSubject::<usize, &'static str>::default();
	subject.error("error");
	assert!(subject.is_closed());
}
#[test]
fn should_be_closed_after_unsubscribe() {
	let mut subject = PublishSubject::<usize, &'static str>::default();
	subject.unsubscribe();
	assert!(subject.is_closed());
}

#[test]
fn teardowns_added_subscriptions_from_a_subject_should_belong_to_the_subscription() {
	let destination = MockObserver::default();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subscription = subject.subscribe(destination);
	let shared_flag = Arc::new(Mutex::new(false));
	let shared_flag_for_teardown = shared_flag.clone();
	subscription.add_fn(move || *shared_flag_for_teardown.lock().unwrap() = true);

	assert!(
		!*shared_flag.lock().unwrap(),
		"teardown executed earlier than should've!"
	);

	subscription.unsubscribe();

	assert!(*shared_flag.lock().unwrap(), "teardown did not execute!");

	subject.unsubscribe();
}

#[test]
fn additional_teardowns_should_immediately_execute_if_the_subscription_is_already_closed() {
	let destination = MockObserver::default();

	let mut subject = PublishSubject::<usize, &'static str>::default();

	let mut subscription = subject.subscribe(destination);
	subscription.unsubscribe();

	let shared_flag = Arc::new(Mutex::new(false));
	let shared_flag_for_teardown = shared_flag.clone();
	subscription.add_fn(move || *shared_flag_for_teardown.lock().unwrap() = true);
	assert!(*shared_flag.lock().unwrap(), "teardown did not execute!");

	subject.unsubscribe();
}
