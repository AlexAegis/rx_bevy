use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_just_forward_nexts() {
	let mut harness =
		TestHarness::<TestSubject<usize, Never>, usize, TestError>::new("map_never (error)");
	let observable = harness.create_harness_observable().map_never();
	harness.subscribe_to(observable);

	harness.source().next(1);
	harness.source().next(2);

	harness.notifications().assert_notifications(
		"map_never (error)",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);
}

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let mut harness = TestHarness::<_, usize, Never>::new("map_never (error)");
		let composed = compose_operator::<usize, Never>().map_never();
		let observable = harness.create_harness_observable().pipe(composed);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}

/// rx_contract_closed_after_error - impossible, error type is never
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, Never>, usize, TestError>::new("map_never (error)");
		let observable = harness.create_harness_observable().map_never();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, Never>, usize, TestError>::new("map_never (error)");
		let observable = harness.create_harness_observable().map_never();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
