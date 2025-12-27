use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[derive(Clone)]
struct Foo;

#[derive(PartialEq, Debug)]
struct Bar;

impl From<Foo> for Bar {
	fn from(_value: Foo) -> Self {
		Bar
	}
}

#[test]
fn should_map_next_emissions_using_the_mapper_provided() {
	let destination = MockObserver::<Bar, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Foo, &'static str>::default();

	let subscription = source.clone().map_into().subscribe(destination);

	source.next(Foo);

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_into",
		0,
		[SubscriberNotification::Next(Bar)],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<Bar, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Foo, &'static str>::default();

	let subscription = source.clone().map_into().subscribe(destination);

	let error = "error";
	source.error(error);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_into",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<Bar, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Foo, &'static str>::default();

	let subscription = source.clone().map_into().subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_into",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<Bar, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Foo, &'static str>::default();

	let composed = compose_operator::<Foo, &'static str>().map_into();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(Foo);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_into",
		0,
		[
			SubscriberNotification::Next(Bar),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
