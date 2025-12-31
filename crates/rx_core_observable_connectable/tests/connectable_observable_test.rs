use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod connector_publish_subject {
	use super::*;

	#[test]
	fn should_not_relay_signals_before_connecting() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<PublishSubject<_, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);

		assert!(
			notification_collector.lock().is_empty(),
			"no notification should've observed yet"
		);
	}

	#[test]
	fn should_relay_signals_after_connecting() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<PublishSubject<_, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		connectable_observable.connect();
		source.next(1);

		notification_collector.lock().assert_notifications(
			"connectable - behavior_subject",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);
	}

	#[test]
	fn should_not_relay_signals_while_disconnected() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<PublishSubject<_, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		connectable_observable.connect();
		assert!(connectable_observable.is_connected());
		source.next(1);
		connectable_observable.disconnect();
		assert!(!connectable_observable.is_connected());
		source.next(2);
		connectable_observable.connect();
		assert!(connectable_observable.is_connected());
		source.next(3);

		notification_collector.lock().assert_notifications(
			"connectable - behavior_subject",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(3),
			],
			true,
		);
	}
}

mod connector_replay_subject {
	use super::*;

	#[test]
	fn should_not_relay_signals_before_connecting_when_its_not_primed() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<ReplaySubject<1, _, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);

		assert!(
			notification_collector.lock().is_empty(),
			"no notification should've observed yet"
		);
	}

	#[test]
	fn should_not_be_able_to_prime_a_replaying_connector_without_connecting() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<ReplaySubject<1, _, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);
		connectable_observable.connect();

		assert!(
			notification_collector.lock().is_empty(),
			"no notification should've observed yet"
		);

		source.next(2);

		notification_collector.lock().assert_notifications(
			"connectable - behavior_subject",
			0,
			[SubscriberNotification::Next(2)],
			true,
		);
	}

	#[test]
	fn should_be_able_to_replay_when_primed_even_if_disconnected() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: ProvideWithDefault::<ReplaySubject<1, _, _>>::default(),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		connectable_observable.connect();
		source.next(1);
		connectable_observable.disconnect();

		assert!(
			notification_collector.lock().is_empty(),
			"no notification should've observed yet"
		);

		let _s = connectable_observable.subscribe(destination);

		notification_collector.lock().assert_notifications(
			"connectable - behavior_subject",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);
	}
}

mod connector_behavior_subject {
	use super::*;

	#[test]
	fn should_not_relay_signals_before_connecting_besides_the_stored_value() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_provider: || BehaviorSubject::new(0),
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);

		notification_collector.lock().assert_notifications(
			"connectable - behavior_subject",
			0,
			[SubscriberNotification::Next(0)],
			true,
		);
	}
}

mod connectable_options {
	use rx_core::prelude::*;

	#[test]
	fn should_default_to_using_the_publish_subject() {
		let default_options = ConnectableOptions::<PublishSubject<_, _>>::default();
		let subject: PublishSubject<usize, &'static str> =
			default_options.connector_provider.provide();
		assert!(!subject.is_closed());
	}
}
