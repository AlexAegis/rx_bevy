use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_be_able_to_subscribe_to_the_source() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let deferred_source = source.clone();
	let mut deferred_observable = DeferredObservable::new(move || deferred_source.clone());

	let _subscription = deferred_observable.subscribe(destination);

	source.next(1);
	source.unsubscribe();

	notification_collector.lock().assert_notifications(
		"deferred_observable",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_be_able_to_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let deferred_source = source.clone();
	let mut deferred_observable = DeferredObservable::new(move || deferred_source.clone());

	let _subscription = deferred_observable.subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"deferred_observable",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_be_able_to_complete_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let deferred_source = source.clone();
	let mut deferred_observable = DeferredObservable::new(move || deferred_source.clone());

	let _subscription = deferred_observable.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"deferred_observable",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

mod observable_fn {
	use super::*;

	#[test]
	fn should_be_able_to_subscribe_to_the_source() {
		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let deferred_source = source.clone();
		let mut deferred_observable = deferred_observable(move || deferred_source.clone());

		let _subscription = deferred_observable.subscribe(destination);

		source.next(1);
		source.unsubscribe();

		notification_collector.lock().assert_notifications(
			"deferred_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut source = PublishSubject::<usize, MockError>::default();
		let mut source_finalized = SharedSubscription::default();
		let source_tracked_teardown = source_finalized.add_tracked_teardown("deferred - source");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"deferred",
			deferred_observable({
				let source = source.clone();
				move || {
					source.clone().finalize({
						let mut source_finalized = source_finalized.clone();
						move || source_finalized.unsubscribe()
					})
				}
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		source.error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));

		source_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut source = PublishSubject::<usize, MockError>::default();
		let mut source_finalized = SharedSubscription::default();
		let source_tracked_teardown = source_finalized.add_tracked_teardown("deferred - source");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"deferred",
			deferred_observable({
				let source = source.clone();
				move || {
					source.clone().finalize({
						let mut source_finalized = source_finalized.clone();
						move || source_finalized.unsubscribe()
					})
				}
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		source.next(1);
		source.complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		source_tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let source = PublishSubject::<usize, MockError>::default();
		let mut source_finalized = SharedSubscription::default();
		let source_tracked_teardown = source_finalized.add_tracked_teardown("deferred - source");

		let mut harness = TestHarness::<_, usize, MockError>::new_with_source(
			"deferred",
			deferred_observable({
				let source = source.clone();
				move || {
					source.clone().finalize({
						let mut source_finalized = source_finalized.clone();
						move || source_finalized.unsubscribe()
					})
				}
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		source_tracked_teardown.assert_was_torn_down();
	}
}
