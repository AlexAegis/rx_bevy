use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Clone)]
enum Either {
	O1,
	O2,
	O3,
}

#[test]
fn should_exhaust_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let subscription = source
		.clone()
		.exhaust_map(
			move |next| match next {
				Either::O1 => (0..=2).into_observable(), // They all complete immediately
				Either::O2 => (3..=4).into_observable(),
				Either::O3 => (5..=6).into_observable(),
			},
			|error| error,
		)
		.subscribe(destination);

	notification_collector.lock().assert_is_empty("exhaust_map");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_map - iterators",
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
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_subscribe_to_the_next_inner_observable_when_there_are_no_active_inner_subscriptions() {
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
		.exhaust_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);

	inner_2.next(0); // Nothing should happen, only inner_1 is subscribed
	inner_3.next(0);
	notification_collector.lock().assert_is_empty("exhaust_map");

	inner_1.next(1);
	inner_1.next(2);

	notification_collector.lock().assert_notifications(
		"exhaust_map - first observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	inner_1.complete();
	source.next(Either::O2);
	inner_2.next(3);

	source.complete();
	inner_3.complete();
	inner_2.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_map - the rest",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_immediately_complete_if_there_are_no_active_subscriptions() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.exhaust_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_map",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}

#[test]
fn should_immediately_error_by_an_inner_error() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let mut inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.exhaust_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	inner_1.next(1);
	let error = "error";
	inner_1.error(error);

	notification_collector.lock().assert_notifications(
		"exhaust_map",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after error"
	);
}

#[test]
fn should_immediately_error_by_an_upstream_error() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Either, &'static str>::default();

	let inner_1 = PublishSubject::<usize, &'static str>::default();
	let inner_2 = PublishSubject::<usize, &'static str>::default();
	let inner_3 = PublishSubject::<usize, &'static str>::default();

	let inner_1_clone = inner_1.clone();
	let inner_2_clone = inner_2.clone();
	let inner_3_clone = inner_3.clone();
	let subscription = source
		.clone()
		.exhaust_map(
			move |next| match next {
				Either::O1 => inner_1_clone.clone(),
				Either::O2 => inner_2_clone.clone(),
				Either::O3 => inner_3_clone.clone(),
			},
			|error| error,
		)
		.subscribe(destination);

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"exhaust_map",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after error"
	);
}

#[test]
fn should_compose_and_exhaust_all_iterators() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<_, _>::default();

	let composed = compose_operator().exhaust_map(
		move |next| match next {
			Either::O1 => (0..=2).into_observable(),
			Either::O2 => (3..=4).into_observable(),
			Either::O3 => (5..=6).into_observable(),
		},
		|error| error,
	);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	notification_collector.lock().assert_is_empty("exhaust_map");

	source.next(Either::O1);
	source.next(Either::O2);
	source.next(Either::O3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"exhaust_map - iterators",
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
		],
		true,
	);

	assert!(
		subscription.is_closed(),
		"subscription should be closed after completion"
	);
}
