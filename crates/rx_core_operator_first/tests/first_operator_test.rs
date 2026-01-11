use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_operator_first::FirstOperatorError;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_and_complete_on_the_first_next() {
	let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().first().subscribe(destination);

	source.next(0);
	assert!(subscription.is_closed());
	source.next(1);

	notification_collector.lock().assert_notifications(
		"first",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_error_if_no_emission_was_observed_before_completion() {
	let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().first().subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"first",
		0,
		[SubscriberNotification::Error(
			FirstOperatorError::NoNextObservedBeforeComplete,
		)],
		true,
	);
}

#[test]
fn should_forward_upstream_errors() {
	let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().first().subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"first",
		0,
		[SubscriberNotification::Error(FirstOperatorError::Upstream(
			error,
		))],
		true,
	);
}

#[test]
fn should_unsubscribe_normally_if_unsubscribed_before_observing_anything() {
	let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().first().subscribe(destination);

	source.unsubscribe();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"first",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_be_composable() {
	let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().first();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	assert!(subscription.is_closed());
	source.next(1);

	notification_collector.lock().assert_notifications(
		"first",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source.clone().first().subscribe(destination);

		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"first - error",
			0,
			[SubscriberNotification::Error(FirstOperatorError::Upstream(
				error,
			))],
			true,
		);
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source.clone().first().subscribe(destination);

		source.complete();

		notification_collector.lock().assert_notifications(
			"first - complete",
			0,
			[SubscriberNotification::Error(
				FirstOperatorError::NoNextObservedBeforeComplete,
			)],
			true,
		);
		assert!(subscription.is_closed());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let destination = MockObserver::<usize, FirstOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source.clone().first().subscribe(destination);

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"first - unsubscribe",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);
		assert!(subscription.is_closed());
	}
}
