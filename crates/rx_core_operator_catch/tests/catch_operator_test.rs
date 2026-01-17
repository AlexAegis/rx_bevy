use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod feature {
	use super::*;

	#[test]
	fn should_catch_an_upstream_error() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<_, _>::default();

		let subscription = source
			.clone()
			.catch(move |_error| just(99))
			.subscribe(destination);

		notification_collector.lock().assert_is_empty("catch");

		let error = "error";
		source.next(1);
		source.next(2);
		source.next(3);
		source.error(error);

		notification_collector.lock().assert_notifications(
			"catch",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Next(99),
				SubscriberNotification::Complete,
			],
			true,
		);

		assert!(
			subscription.is_closed(),
			"subscription should be closed after completion"
		);
	}
}

mod teardown {
	use super::*;

	#[test]
	fn should_catch_an_upstream_error() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<_, _>::default();

		let mut tracked_subscription = SharedSubscription::default();
		let inner_teardown_tracker = tracked_subscription.add_tracked_teardown("catch - inner");

		let mut subscription = source
			.clone()
			.catch(move |_error| {
				just(99).finalize(move || {
					tracked_subscription.unsubscribe();
				})
			})
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("catch - outer");

		notification_collector.lock().assert_is_empty("catch");

		let error = "error";
		source.next(1);
		source.next(2);
		source.next(3);
		source.error(error);

		notification_collector.lock().assert_notifications(
			"catch",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Next(99),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		inner_teardown_tracker.assert_was_torn_down();

		assert!(
			subscription.is_closed(),
			"subscription should be closed after completion"
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, Never>::new("catch");
		let observable = harness.create_harness_observable().catch(|_error| just(99));
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error("error");
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, Never>::new("catch");
		let observable = harness.create_harness_observable().catch(|_error| just(99));
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, &'static str>, usize, Never>::new("catch");
		let observable = harness.create_harness_observable().catch(|_error| just(99));
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
