use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_take_the_first_n_emissions_then_complete() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().take(2).subscribe(destination);

	source.next(0);
	source.next(1);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"take",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod given_two_take_operators {
	use super::*;

	mod when_they_are_equal {
		use super::*;

		#[test]
		fn should_complete_when_the_take_count_is_reached() {
			let destination = MockObserver::<usize, &'static str>::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();

			let subscription = source.clone().take(2).take(2).subscribe(destination);

			source.next(0);
			source.next(1);

			assert!(subscription.is_closed());

			notification_collector.lock().assert_notifications(
				"take",
				0,
				[
					SubscriberNotification::Next(0),
					SubscriberNotification::Next(1),
					SubscriberNotification::Complete,
				],
				true,
			);
		}
	}

	mod when_upstream_is_smaller {
		use super::*;

		#[test]
		fn should_complete_when_the_smaller_take_count_is_reached() {
			let destination = MockObserver::<usize, &'static str>::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();

			let subscription = source.clone().take(1).take(2).subscribe(destination);

			source.next(0);
			source.next(1);

			assert!(subscription.is_closed());

			notification_collector.lock().assert_notifications(
				"take",
				0,
				[
					SubscriberNotification::Next(0),
					SubscriberNotification::Complete,
				],
				true,
			);
		}
	}

	mod when_downstream_is_smaller {
		use super::*;

		#[test]
		fn should_complete_when_the_smaller_take_count_is_reached() {
			let destination = MockObserver::<usize, &'static str>::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();

			let subscription = source.clone().take(2).take(1).subscribe(destination);

			source.next(0);
			source.next(1);

			assert!(subscription.is_closed());

			notification_collector.lock().assert_notifications(
				"take",
				0,
				[
					SubscriberNotification::Next(0),
					SubscriberNotification::Complete,
				],
				true,
			);
		}
	}
}

#[test]
fn should_immediately_complete_and_unsubscribe_if_take_count_is_zero() {
	let destination = MockObserver::<Never>::default();
	let notification_collector = destination.get_notification_collector();

	let subscription = never().take(0).subscribe(destination);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"take",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().take(2).subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("take");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"take",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().take(2).subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("take");

	source.complete();

	notification_collector.lock().assert_notifications(
		"take",
		0,
		[SubscriberNotification::Complete],
		true,
	);
	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().take(2);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"take",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}
