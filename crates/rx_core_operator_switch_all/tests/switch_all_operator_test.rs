use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::SubscriberNotification;

#[test]
fn subscribes_to_the_inner_observable_as_many_times_as_many_upstream_emissions_there_are() {
	let mock_destination = MockObserver::<i32>::default();
	let notification_collector = mock_destination.get_notification_collector();

	let mut source = (1..=2)
		.into_observable()
		.map(|_| (10..=12).into_observable())
		.switch_all(|_| unreachable!());
	let subscription = source.subscribe(mock_destination);

	notification_collector.lock().assert_notifications(
		"switch_all",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(11),
			SubscriberNotification::Next(12),
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(11),
			SubscriberNotification::Next(12),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn subscribes_to_the_inner_observable_on_every_emit_of_a_source_subject_and_completes() {
	let mock_destination = MockObserver::<i32>::default();
	let notification_collector = mock_destination.get_notification_collector();

	let mut subject = PublishSubject::<i32, Never>::default();
	let mut source = subject
		.clone()
		.map(|i| (0..=i).into_observable())
		.switch_all(|_| unreachable!());
	let mut subscription = source.subscribe(mock_destination);

	subject.next(1);

	assert_eq!(
		notification_collector.lock().all_observed_values(),
		vec![0, 1]
	);

	subject.next(3);
	assert_eq!(
		notification_collector.lock().all_observed_values(),
		vec![0, 1, 0, 1, 2, 3]
	);

	subject.complete();

	assert!(matches!(
		notification_collector.lock().nth_notification(6),
		&SubscriberNotification::Complete
	));

	subscription.unsubscribe();

	assert!(matches!(
		notification_collector.lock().nth_notification(7),
		&SubscriberNotification::Unsubscribe
	));

	subject.unsubscribe();
}

#[test]
fn upstream_ticks_are_forwarded_to_the_inner_subscription() {
	let mock_destination = MockObserver::<i32>::default();
	let notification_collector = mock_destination.get_notification_collector();

	let mut subject = PublishSubject::<i32, Never>::default();
	let mut source = subject
		.clone()
		.map(|i| (0..=i).into_observable())
		.switch_all(Never::error_mapper());
	let mut subscription = source.subscribe(mock_destination);

	subject.next(1);
	assert_eq!(
		notification_collector.lock().all_observed_values(),
		vec![0, 1]
	);

	subject.next(3);
	assert_eq!(
		notification_collector.lock().all_observed_values(),
		vec![0, 1, 0, 1, 2, 3]
	);

	subject.unsubscribe();
	subscription.unsubscribe();
}
