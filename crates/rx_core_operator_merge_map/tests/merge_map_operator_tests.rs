use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Clone)]
enum Either {
	O1,
	O2,
	O3,
}

#[test]
fn should_merge_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.merge_map(
			move |next| match next {
				Either::O1 => (0..=2).into_observable(),
				Either::O2 => (3..=4).into_observable(),
				Either::O3 => (5..=6).into_observable(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("merge_all");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
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
fn should_subscribe_to_all_of_the_inner_observables_if_the_limit_allows_it() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			usize::MAX,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_is_empty("merge_map");

	inner_2.next(0);
	inner_3.next(1);
	inner_1.next(2);
	inner_1.next(3);

	notification_collector.lock().assert_notifications(
		"merge_map - first observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
		],
		true,
	);

	inner_1.complete();
	inner_2.next(4);
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"merge_map - the rest",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
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
fn should_subscribe_to_as_many_of_the_inner_observables_as_the_limit_allows_it() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let mut inner_2 = PublishSubject::<usize, &'static str>::default();
	let mut inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.merge_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			2,
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	inner_3.next(1); // Nothing should happen as only 2 can be subscribed at a time

	notification_collector.lock().assert_is_empty("merge_map");

	inner_2.next(0);
	inner_1.next(1);
	inner_1.next(2);

	notification_collector.lock().assert_notifications(
		"merge_map - first observable",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	inner_3.next(3);
	inner_3.complete();
	inner_2.next(4);
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"merge_map - the rest",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(4),
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
