use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_concat_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.concat_all(|error| error)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next((0..=2).into_observable());
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_all - iterators",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_subscribe_to_the_inner_observables_one_at_a_time() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<PublishSubject<usize, &'static str>, &'static str>::default();

	let subscription = source
		.clone()
		.concat_all(|error| error)
		.subscribe(destination);

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	source.next(inner_1.clone());
	source.next(inner_2.clone());
	source.next(inner_3.clone());
	source.complete();

	inner_2.next(0); // Nothing should happen, only inner_1 is subscribed
	inner_3.next(0);
	notification_collector.lock().assert_is_empty("concat_all");

	inner_1.next(1);
	inner_1.next(2);

	notification_collector.lock().assert_notifications(
		"concat_all - first observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	inner_2.next(3);
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"concat_all - the rest",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_compose_and_concat_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let composed = compose_operator().concat_all(|error| error);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next((0..=2).into_observable());
	source.next((3..=4).into_observable());
	source.next((5..=6).into_observable());
	source.complete();

	notification_collector.lock().assert_notifications(
		"merge_all - iterators",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(5),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}
