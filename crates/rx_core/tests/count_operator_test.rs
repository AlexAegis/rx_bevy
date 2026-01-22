use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

mod when_counting {
	use super::*;

	#[test]
	fn should_emit_total_on_complete() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let _subscription = source.clone().count().subscribe(destination);

		source.next(0);
		source.next(0);
		source.next(0);
		source.complete();

		notification_collector.lock().assert_notifications(
			"count",
			0,
			[
				SubscriberNotification::Next(3),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_emit_zero_when_no_values_were_observed() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let _subscription = source.clone().count().subscribe(destination);

		source.complete();

		notification_collector.lock().assert_notifications(
			"count",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Complete,
			],
			true,
		);
	}

	#[test]
	fn should_forward_errors() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let _subscription = source.clone().count().subscribe(destination);

		source.next(1);
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"count",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);
	}

	#[test]
	fn should_compose() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let composed = compose_operator::<usize, &'static str>().count();
		let _subscription = source.clone().pipe(composed).subscribe(destination);

		source.next(1);
		source.next(2);
		source.complete();

		notification_collector.lock().assert_notifications(
			"count",
			0,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("count");
		let observable = harness.create_harness_observable().count();
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("count");
		let observable = harness.create_harness_observable().count();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("count");
		let observable = harness.create_harness_observable().count();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
