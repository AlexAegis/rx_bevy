use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_complete_if_all_inputs_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<usize>::default();
	let mut subject_3 = PublishSubject::<usize>::default();

	let mut subscription =
		ConcatObservable::new((subject_1.clone(), subject_2.clone(), subject_3.clone()))
			.subscribe(destination);

	assert!(
		notification_collector.lock().is_empty(),
		"nothing should happen when subscribed to non replaying sources"
	);

	subject_1.next(1);

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Next(1)
	);

	subject_2.next(2);

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"should not be subscribed to the second source until the first one completes"
	);

	subject_1.complete();
	subject_3.complete(); // This will never emit

	subject_2.next(2);
	subject_2.next(3);
	subject_2.complete();

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concat",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_immediately_complete_all_inputs_immediately_complete() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut subject_1 = PublishSubject::<usize>::default();
	subject_1.complete();
	let mut subject_2 = PublishSubject::<usize>::default();
	subject_2.complete();

	let mut subscription = concat((subject_1.clone(), subject_2.clone())).subscribe(destination);

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"concat",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}
