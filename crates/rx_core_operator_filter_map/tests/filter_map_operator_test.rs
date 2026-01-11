use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_map_and_filter_next_emissions_using_the_mapper_provided_filtering_when_none() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter_map(|i| {
			if i % 2 == 0 {
				Some(format!("{i}"))
			} else {
				None
			}
		})
		.subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.next(4);

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter_map",
		0,
		[
			SubscriberNotification::Next("2".to_string()),
			SubscriberNotification::Next("4".to_string()),
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter_map(|i| {
			if i % 2 == 0 {
				Some(format!("{i}"))
			} else {
				None
			}
		})
		.subscribe(destination);

	let error = "error";
	source.error(error);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter_map",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter_map(|i| {
			if i % 2 == 0 {
				Some(format!("{i}"))
			} else {
				None
			}
		})
		.subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter_map",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().filter_map(|i| {
		if i % 2 == 0 {
			Some(format!("{i}"))
		} else {
			None
		}
	});

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter_map",
		0,
		[
			SubscriberNotification::Next("2".to_string()),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, String, TestError>::new("filter_map");
		let observable = harness.create_harness_observable().filter_map(|value| {
			if value % 2 == 0 {
				Some(format!("{value}"))
			} else {
				None
			}
		});
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, String, TestError>::new("filter_map");
		let observable = harness.create_harness_observable().filter_map(|value| {
			if value % 2 == 0 {
				Some(format!("{value}"))
			} else {
				None
			}
		});
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, String, TestError>::new("filter_map");
		let observable = harness.create_harness_observable().filter_map(|value| {
			if value % 2 == 0 {
				Some(format!("{value}"))
			} else {
				None
			}
		});
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
