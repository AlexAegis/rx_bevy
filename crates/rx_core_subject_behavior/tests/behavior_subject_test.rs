use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_replay_its_value_to_new_subscribers() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let behavior_subject = BehaviorSubject::<usize>::new(1);

	let _s = behavior_subject.clone().subscribe(destination_1);

	notification_collector_1.lock().assert_notifications(
		"behavior_subject",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);
}

#[test]
fn should_be_able_to_default_if_the_input_type_can() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let behavior_subject = BehaviorSubject::<usize>::default();

	let _s = behavior_subject.clone().subscribe(destination_1);

	notification_collector_1.lock().assert_notifications(
		"behavior_subject",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);
}

#[test]
fn should_continue_to_multicast() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut behavior_subject = BehaviorSubject::<usize>::new(1);

	let _s = behavior_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(1),
		"destination_1 did not receive the replay"
	);

	let destination_2 = MockObserver::default();
	let notification_collector_2 = destination_2.get_notification_collector();

	let _s = behavior_subject.clone().subscribe(destination_2);

	assert_eq!(
		notification_collector_2.lock().nth_notification(0),
		&SubscriberNotification::Next(1),
		"destination_2 did not receive the replay"
	);

	behavior_subject.next(2);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Next(2),
		"destination_1 did not receive the updated value"
	);

	assert_eq!(
		notification_collector_2.lock().nth_notification(1),
		&SubscriberNotification::Next(2),
		"destination_2 did not receive the updated value"
	);
}

#[test]
fn should_emit_the_last_value_after_completed_for_late_subscribers() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut behavior_subject = BehaviorSubject::<usize>::new(1);

	behavior_subject.next(0);
	behavior_subject.next(1);
	behavior_subject.next(2); // The last observed value
	behavior_subject.complete();

	let _s = behavior_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(2),
		"destination did not receive the replay"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Complete,
		"destination did not receive the completion signal"
	);

	behavior_subject.unsubscribe();

	assert_eq!(
		notification_collector_1.lock().nth_notification(2),
		&SubscriberNotification::Unsubscribe,
		"destination did not receive the unsubscribe signal"
	);
}

#[test]
fn should_not_emit_the_last_value_after_errored() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut behavior_subject = BehaviorSubject::<usize, &'static str>::new(1);
	let error = "error";
	behavior_subject.next(0);
	behavior_subject.next(1);
	behavior_subject.error(error);

	let _s = behavior_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Error(error),
		"destination did not receive the error"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Unsubscribe,
		"destination did not receive the unsubscribe signal"
	);
}

#[test]
fn should_let_the_last_value_to_be_read_without_subscribing() {
	let mut behavior_subject = BehaviorSubject::<usize, &'static str>::new(1);

	assert_eq!(behavior_subject.value(), 1);

	behavior_subject.next(2);

	assert_eq!(behavior_subject.value(), 2);
}

#[test]
fn should_be_closed_after_completion() {
	let mut subject = BehaviorSubject::<usize, &'static str>::new(1);
	subject.complete();
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_error() {
	let mut subject = BehaviorSubject::<usize, &'static str>::new(1);
	subject.error("error");
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_unsubscribe() {
	let mut subject = BehaviorSubject::<usize, &'static str>::new(1);
	subject.unsubscribe();
	assert!(subject.is_closed());
}
