use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod when_connected {
	use super::*;

	#[test]
	fn should_not_unsubsribe_downstream_when_the_source_unsubscribes() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
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
		source.unsubscribe();

		source.next(2);
		assert!(!connectable_observable.is_connected());

		notification_collector.lock().assert_notifications(
			"connectable - publish_subject",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);
	}

	#[test]
	fn should_send_complete_downstream_when_the_source_completes() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		connectable_observable.connect();
		source.next(1);
		source.complete();
		source.next(2);

		notification_collector.lock().assert_notifications(
			"connectable - publish_subject",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}

	#[test]
	fn should_send_error_downstream_when_the_source_errors() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		connectable_observable.connect();
		source.next(1);
		let error = "error";
		source.error(error);
		source.next(2);

		notification_collector.lock().assert_notifications(
			"connectable - publish_subject",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Error(error),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod when_disconnected {
	use super::*;

	#[test]
	fn should_not_unsubsribe_downstream_when_the_source_unsubscribes_and_disconnected() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);
		source.unsubscribe();
		source.next(2);

		assert!(!connectable_observable.is_connected());

		notification_collector.lock().assert_notifications(
			"connectable - publish_subject",
			0,
			[],
			true,
		);
	}

	#[test]
	fn should_not_send_complete_downstream_when_the_source_completes() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);
		source.complete();
		source.next(2);

		notification_collector.lock().assert_is_empty("connectable");
	}

	#[test]
	fn should_not_send_error_downstream_when_the_source_errors() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: PublishSubject::default,
				disconnect_when_ref_count_zero: false,
				reset_connector_on_disconnect: false,
				reset_connector_on_complete: false,
				reset_connector_on_error: false,
			},
		);

		let _s = connectable_observable.subscribe(destination);

		source.next(1);
		let error = "error";
		source.error(error);
		source.next(2);

		notification_collector.lock().assert_is_empty("connectable");
	}
}
