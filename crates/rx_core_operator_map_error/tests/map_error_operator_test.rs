use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_map_error_next_emissions_using_the_error_mapper_provided() {
	let destination = MockObserver::<usize, String>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.map_error(|error| format!("mapped {error}"))
		.subscribe(destination);

	let error = "error";
	source.next(0);
	source.next(1);
	source.error(error);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_error",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(format!("mapped {error}")),
		],
		true,
	);
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<usize, String>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.map_error(|error| format!("mapped {error}"))
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("map_error");

	let error = "error";
	source.next(0);
	source.error(error);

	notification_collector.lock().assert_notifications(
		"map_error",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Error(format!("mapped {error}")),
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<usize, String>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.map_error(|error| format!("mapped {error}"))
		.subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("map_error");

	source.complete();

	notification_collector.lock().assert_notifications(
		"map_error",
		0,
		[SubscriberNotification::Complete],
		true,
	);
	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, String>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed =
		compose_operator::<usize, &'static str>().map_error(|error| format!("mapped {error}"));

	let subscription = source.clone().pipe(composed).subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map_error",
		0,
		[SubscriberNotification::Error(format!("mapped {error}"))],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, String>::new("map_error");
		let observable = harness
			.create_harness_observable()
			.map_error(|error| format!("mapped {error}"));
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error("error");
		harness.assert_terminal_notification(SubscriberNotification::Error(
			"mapped error".to_string(),
		));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, String>::new("map_error");
		let observable = harness
			.create_harness_observable()
			.map_error(|error| format!("mapped {error}"));
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, String>::new("map_error");
		let observable = harness
			.create_harness_observable()
			.map_error(|error| format!("mapped {error}"));
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
