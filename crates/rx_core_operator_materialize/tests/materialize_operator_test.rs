use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_turn_next_emissions_into_notifications() {
	let destination = MockObserver::<ObserverNotification<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().materialize().subscribe(destination);

	source.next(0);
	source.next(1);

	notification_collector.lock().assert_notifications(
		"materialize",
		0,
		[
			SubscriberNotification::Next(ObserverNotification::Next(0)),
			SubscriberNotification::Next(ObserverNotification::Next(1)),
		],
		true,
	);
}

#[test]
fn should_turn_error_emissions_into_notifications_and_not_error() {
	let destination = MockObserver::<ObserverNotification<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().materialize().subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"materialize",
		0,
		[
			SubscriberNotification::Next(ObserverNotification::Error(error)),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_turn_complete_emissions_into_notifications_and_not_complete() {
	let destination = MockObserver::<ObserverNotification<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().materialize().subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"materialize",
		0,
		[
			SubscriberNotification::Next(ObserverNotification::Complete),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_simply_unsubscribe_when_the_source_unsubscribes() {
	let destination = MockObserver::<ObserverNotification<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().materialize().subscribe(destination);

	source.unsubscribe();

	notification_collector.lock().assert_notifications(
		"materialize",
		0,
		[
			SubscriberNotification::Unsubscribe, // From the subject
		],
		true,
	);
}

#[test]
fn should_be_composable() {
	let destination = MockObserver::<ObserverNotification<usize, &'static str>, Never>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().materialize();

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.complete();

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[
			SubscriberNotification::Next(ObserverNotification::Next(1)),
			SubscriberNotification::Next(ObserverNotification::Complete),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<
			TestSubject<usize, TestError>,
			ObserverNotification<usize, TestError>,
			Never,
		>::new("materialize");
		let observable = harness.create_harness_observable().materialize();
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<
			TestSubject<usize, &'static str>,
			ObserverNotification<usize, &'static str>,
			Never,
		>::new("materialize");
		let observable = harness.create_harness_observable().materialize();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<
			TestSubject<usize, &'static str>,
			ObserverNotification<usize, &'static str>,
			Never,
		>::new("materialize");
		let observable = harness.create_harness_observable().materialize();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
