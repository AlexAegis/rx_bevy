use rx_core::prelude::*;
use rx_core_common::{Observer, SubscriptionLike};
use rx_core_testing::prelude::*;

#[test]
fn should_not_replay_if_there_is_nothing_to_replay() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let replay_subject = ReplaySubject::<1, usize>::default();
	let _s = replay_subject.clone().subscribe(destination_1);

	assert!(
		!notification_collector_1.lock().nth_notification_exists(0),
		"destination received something when it shouldn't have"
	);
}

#[test]
fn should_replay_its_last_value_to_new_subscribers() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut replay_subject = ReplaySubject::<1, usize>::default();
	replay_subject.next(10);
	let _s = replay_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(10),
		"destination did not receive the replay"
	);
}

#[test]
fn should_replay_multiple_values_to_new_subscribers_with_larger_capacity() {
	let destination_1 = MockObserver::default();
	let notification_collector_1 = destination_1.get_notification_collector();

	let mut replay_subject = ReplaySubject::<3, usize>::default();
	replay_subject.next(10);
	replay_subject.next(11);
	let _s = replay_subject.clone().subscribe(destination_1);

	assert_eq!(
		notification_collector_1.lock().nth_notification(0),
		&SubscriberNotification::Next(10),
		"destination_1 did not receive the replay"
	);

	assert_eq!(
		notification_collector_1.lock().nth_notification(1),
		&SubscriberNotification::Next(11),
		"destination_1 did not receive the replay"
	);

	assert!(
		!notification_collector_1.lock().nth_notification_exists(2),
		"destination_1 received an additional replay when there should've been none"
	);

	replay_subject.next(12);

	assert_eq!(
		notification_collector_1.lock().nth_notification(2),
		&SubscriberNotification::Next(12),
		"destination_1 did not receive the new, non-replayed value"
	);

	replay_subject.next(13);

	assert_eq!(
		notification_collector_1.lock().nth_notification(3),
		&SubscriberNotification::Next(13),
		"destination_1 did not receive the new, non-replayed value"
	);

	let destination_2 = MockObserver::default();
	let notification_collector_2 = destination_2.get_notification_collector();
	let _s = replay_subject.clone().subscribe(destination_2);

	assert_eq!(
		notification_collector_2.lock().nth_notification(0),
		&SubscriberNotification::Next(11),
		"destination_2 did not receive the replay"
	);

	assert_eq!(
		notification_collector_2.lock().nth_notification(1),
		&SubscriberNotification::Next(12),
		"destination_2 did not receive the replay"
	);

	assert_eq!(
		notification_collector_2.lock().nth_notification(2),
		&SubscriberNotification::Next(13),
		"destination_2 did not receive the replay"
	);

	assert!(
		!notification_collector_2.lock().nth_notification_exists(3),
		"destination_2 received an additional replay when there should've been none"
	);
}

#[test]
fn should_replay_its_last_value_to_new_subscribers_after_completed() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut replay_subject = ReplaySubject::<1, usize>::default();
	replay_subject.next(10);
	replay_subject.complete();
	let _s = replay_subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"subject_replay",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_not_replay_its_last_value_to_new_subscribers_after_errored() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut replay_subject = ReplaySubject::<1, usize, &'static str>::default();
	replay_subject.next(10);
	let error = "error";
	replay_subject.error(error);

	let _s = replay_subject.clone().subscribe(destination);

	notification_collector.lock().assert_notifications(
		"subject_replay",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_allow_its_buffer_to_be_read_without_subscribe() {
	let mut replay_subject = ReplaySubject::<2, usize, &'static str>::default();
	assert_eq!(replay_subject.values(), Vec::new());
	replay_subject.next(10);
	assert_eq!(replay_subject.values(), vec![10]);
	replay_subject.next(20);
	assert_eq!(replay_subject.values(), vec![10, 20]);
	replay_subject.next(30);
	assert_eq!(replay_subject.values(), vec![20, 30]);
}

#[test]
fn should_be_closed_after_completion() {
	let mut subject = ReplaySubject::<1, usize, &'static str>::default();
	subject.complete();
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_error() {
	let mut subject = ReplaySubject::<1, usize, &'static str>::default();
	subject.error("error");
	assert!(subject.is_closed());
}

#[test]
fn should_be_closed_after_unsubscribe() {
	let mut subject = ReplaySubject::<1, usize, &'static str>::default();
	subject.unsubscribe();
	assert!(subject.is_closed());
}
