use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_last_value_after_completed_and_unsubscribe() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::default();

	async_subject.next(0); // An observed value, but not the last one

	let _s = async_subject.clone().subscribe(destination);

	assert!(
		notification_collector.lock().is_empty(),
		"Nothing should've been replayed"
	);

	async_subject.next(1);

	assert!(
		notification_collector.lock().is_empty(),
		"Nothing should've been sent before completion"
	);

	async_subject.next(2); // The last observed value
	async_subject.complete();

	notification_collector.lock().assert_notifications(
		"subject_async",
		0,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);

	async_subject.unsubscribe();

	assert!(
		!notification_collector.lock().nth_notification_exists(3),
		"destination should not have received another notification"
	);
}

#[test]
fn should_emit_the_last_value_after_completed_for_late_subscribers() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::default();

	async_subject.next(0);
	async_subject.next(1);
	async_subject.next(2); // The last observed value
	async_subject.complete();

	let _s = async_subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"subject_async",
		0,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
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
	let notification_collector = destination.get_notification_collector();

	let mut async_subject = AsyncSubject::<usize>::new(|acc, next| acc + next);

	async_subject.next(1);
	async_subject.next(2);
	async_subject.next(3);

	let _subscription = async_subject.clone().subscribe(destination);
	async_subject.complete();

	notification_collector.lock().assert_notifications(
		"subject_async",
		0,
		[
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
		],
		true,
	);
}
