use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_found_values_index_and_complete() {
	let destination = MockObserver::<usize, FindIndexOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.find_index(|next| next == &90)
		.subscribe(destination);

	source.next(99);
	source.next(90);
	source.next(20);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find_index",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, FindIndexOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().find_index(|next| next == &20);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(99);
	source.next(90);
	source.next(20);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find_index",
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
	let destination = MockObserver::<usize, FindIndexOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.find_index(|next| next == &2)
		.subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"find_index",
		0,
		[SubscriberNotification::Error(
			FindIndexOperatorError::Upstream(error),
		)],
		true,
	);
}

mod no_match_observed_error {
	use super::*;

	#[test]
	fn should_error_when_completing_before_the_result_was_found_but_notifications_were_observed() {
		let destination = MockObserver::<usize, FindIndexOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let subscription = source
			.clone()
			.find_index(|next| next == &2)
			.subscribe(destination);

		source.next(0);
		source.complete();
		assert!(subscription.is_closed());

		notification_collector.lock().assert_notifications(
			"find_index",
			0,
			[SubscriberNotification::Error(
				FindIndexOperatorError::NoMatchObserved,
			)],
			true,
		);
	}
}

mod no_next_observed_error {
	use super::*;

	#[test]
	fn should_error_when_completing_before_any_value_was_even_observed() {
		let destination = MockObserver::<usize, FindIndexOperatorError<&'static str>>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();

		let subscription = source
			.clone()
			.find_index(|next| next == &2)
			.subscribe(destination);

		source.complete();
		assert!(subscription.is_closed());

		notification_collector.lock().assert_notifications(
			"find_index",
			0,
			[SubscriberNotification::Error(
				FindIndexOperatorError::NoNextObservedBeforeComplete,
			)],
			true,
		);
	}
}
