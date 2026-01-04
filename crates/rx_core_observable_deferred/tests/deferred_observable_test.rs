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
