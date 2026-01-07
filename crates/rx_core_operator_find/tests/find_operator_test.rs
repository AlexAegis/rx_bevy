use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_found_value_and_complete() {
	let destination = MockObserver::<usize, FindOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.find(|next| next == &2)
		.subscribe(destination);

	source.next(0);
	source.next(1);
	source.next(2);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find",
		0,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, FindOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().find(|next| next == &2);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	source.next(1);
	source.next(2);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find",
		0,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_forward_upstream_errors_wrapped() {
	let destination = MockObserver::<usize, FindOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.find(|next| next == &2)
		.subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find",
		0,
		[SubscriberNotification::Error(FindOperatorError::Upstream(
			error,
		))],
		true,
	);
}

mod no_match_observed_error {
	use super::*;

	#[test]
	fn should_error_when_completing_before_the_result_was_found_but_notifications_were_observed() {
		let destination = MockObserver::<usize, FindOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let subscription = source
			.clone()
			.find(|next| next == &2)
			.subscribe(destination);

		source.next(0);
		source.complete();
		assert!(subscription.is_closed());

		notification_collector.lock().assert_notifications(
			"find",
			0,
			[SubscriberNotification::Error(
				FindOperatorError::NoMatchObserved,
			)],
			true,
		);
	}
}

mod no_next_observed_error {
	use super::*;

	#[test]
	fn should_error_when_completing_before_any_value_was_even_observed() {
		let destination = MockObserver::<usize, FindOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let subscription = source
			.clone()
			.find(|next| next == &2)
			.subscribe(destination);

		source.complete();
		assert!(subscription.is_closed());

		notification_collector.lock().assert_notifications(
			"find",
			0,
			[SubscriberNotification::Error(
				FindOperatorError::NoNextObservedBeforeComplete,
			)],
			true,
		);
	}
}
