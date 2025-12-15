use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_last_value_after_completed_and_unsubscribe() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::default();

	async_subject.next(0); // An observed value, but not the last one

	let _s = async_subject.clone().subscribe(destination_1);

	assert!(
		notification_collector_1.lock().is_empty(),
		"Nothing should've been replayed"
	);

	async_subject.next(1);

	assert!(
		notification_collector_1.lock().is_empty(),
		"Nothing should've been sent before completion"
	);

	async_subject.next(2); // The last observed value
	async_subject.complete();

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(2),
		"destination did not receive the result emission"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Complete,
		"destination did not receive the completion signal"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(2),
		&SubscriberNotification::Unsubscribe,
		"destination_1 did not receive the unsubscribe signal"
	);

	async_subject.unsubscribe();

	assert!(
		!notification_collector_1.lock().nth_notification_exists(3),
		"destination should not have recieved another notification"
	);
}

#[test]
fn should_emit_the_last_value_after_completed_for_late_subscribers() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::default();

	async_subject.next(0);
	async_subject.next(1);
	async_subject.next(2); // The last observed value
	async_subject.complete();

	let _s = async_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(2),
		"destination did not receive the result emission"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Complete,
		"destination did not receive the completion signal"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(2),
		&SubscriberNotification::Unsubscribe,
		"destination did not receive the unsubscribe signal"
	);
}

#[test]
fn should_be_closed_after_completion() {
	let mut subject = AsyncSubject::<usize>::default();
	subject.complete();
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_error() {
	let mut subject = AsyncSubject::<usize, &'static str>::default();
	subject.error("error");
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_unsubscribe() {
	let mut subject = AsyncSubject::<usize>::default();
	subject.unsubscribe();
	assert!(subject.is_closed());
}

#[test]
fn should_be_able_to_read_its_latest_value() {
	let mut async_subject = AsyncSubject::<usize>::default();
	assert!(async_subject.value().is_none());
	async_subject.next(1);
	assert!(matches!(async_subject.value(), Some(1)));
}

#[test]
fn should_be_able_to_use_a_custom_reducer_to_accumulate_the_result() {
	let destination = MockObserver::default();
	let notifications = destination.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::new(|acc, next| acc + next);

	async_subject.next(1);
	async_subject.next(2);
	async_subject.next(3);

	let _subscription = async_subject.clone().subscribe(destination);
	async_subject.complete();

	assert_eq!(
		notifications.lock().nth_notification(0),
		&SubscriberNotification::Next(6),
		"destination did not receive the reduced emission",
	);

	assert_eq!(
		notifications.lock().nth_notification(1),
		&SubscriberNotification::Complete,
		"destination did not receive the completion signal",
	);
}
