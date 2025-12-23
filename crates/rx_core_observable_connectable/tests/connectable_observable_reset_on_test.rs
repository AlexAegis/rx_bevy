use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod manual_reset {
	use super::*;

	#[test]
	fn should_reset_when_reset_manually() {
		let destination = MockObserver::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
		// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
		let mut connectable_observable = ConnectableObservable::new(
			source.clone(),
			ConnectableOptions {
				connector_creator: ReplaySubject::<1, _, _>::default,
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
		connectable_observable.reset();

		assert!(!connectable_observable.is_connected());

		notification_collector.lock().assert_notifications(
			"connectable - reset_on_complete 1",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		// Asserting reset by checking if the replay is still primed or not
		let destination_2 = MockObserver::default();
		let notification_collector_2 = destination_2.get_notification_collector();
		let _s2 = connectable_observable.subscribe(destination_2);
		notification_collector_2
			.lock()
			.assert_is_empty("connectable - reset_on_complete 2");
	}
}

mod reset_on_disconnect {

	use super::*;

	mod when_enabled {
		use super::*;

		#[test]
		fn should_reset_when_disconnected_manually() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();
			// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
			// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
					disconnect_when_ref_count_zero: false,
					reset_connector_on_disconnect: true,
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

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_complete 1",
				0,
				[SubscriberNotification::Next(1)],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);
			notification_collector_2
				.lock()
				.assert_is_empty("connectable - reset_on_complete 2");
		}
	}

	mod when_disabled {
		use super::*;

		#[test]
		fn should_not_reset_when_disconnected_manually() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();
			// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
			// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
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

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_complete 1",
				0,
				[SubscriberNotification::Next(1)],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);

			notification_collector_2.lock().assert_notifications(
				"connectable - reset_on_complete 2",
				0,
				[SubscriberNotification::Next(1)],
				true,
			);
		}
	}
}

mod reset_on_complete {

	use super::*;

	mod when_enabled {
		use super::*;

		#[test]
		fn should_reset_when_the_source_completes() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();
			// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
			// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
					disconnect_when_ref_count_zero: false,
					reset_connector_on_disconnect: false,
					reset_connector_on_complete: true,
					reset_connector_on_error: false,
				},
			);

			let _s = connectable_observable.subscribe(destination);

			connectable_observable.connect();
			assert!(connectable_observable.is_connected());

			source.next(1);
			source.complete();

			assert!(!connectable_observable.is_connected());

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_complete 1",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Complete,
					SubscriberNotification::Unsubscribe,
				],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);
			notification_collector_2
				.lock()
				.assert_is_empty("connectable - reset_on_complete 2");
		}
	}

	mod when_disabled {
		use super::*;

		#[test]
		fn should_not_reset_when_the_source_completes() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();
			// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
			// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
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
			source.complete();

			assert!(!connectable_observable.is_connected());

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_complete 1",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Complete,
					SubscriberNotification::Unsubscribe,
				],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);

			notification_collector_2.lock().assert_notifications(
				"connectable - reset_on_complete 2",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Complete,
					SubscriberNotification::Unsubscribe,
				],
				true,
			);
		}
	}
}

mod reset_on_error {

	use super::*;

	mod when_enabled {
		use super::*;

		#[test]
		fn should_reset_when_the_source_errors() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();

			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
					disconnect_when_ref_count_zero: false,
					reset_connector_on_disconnect: false,
					reset_connector_on_complete: false,
					reset_connector_on_error: true,
				},
			);

			let _s = connectable_observable.subscribe(destination);

			connectable_observable.connect();
			assert!(connectable_observable.is_connected());

			source.next(1);
			let error = "error";
			source.error(error);

			assert!(!connectable_observable.is_connected());

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_error 1",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Error(error),
					SubscriberNotification::Unsubscribe,
				],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);
			notification_collector_2
				.lock()
				.assert_is_empty("connectable - reset_on_error 2");
		}
	}

	mod when_disabled {
		use super::*;

		#[test]
		fn should_not_reset_when_the_source_errors() {
			let destination = MockObserver::default();
			let notification_collector = destination.get_notification_collector();

			let mut source = PublishSubject::<usize, &'static str>::default();
			// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
			// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: ReplaySubject::<1, _, _>::default,
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
			let error = "error";
			source.error(error);

			assert!(!connectable_observable.is_connected());

			notification_collector.lock().assert_notifications(
				"connectable - reset_on_error 1",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Error(error),
					SubscriberNotification::Unsubscribe,
				],
				true,
			);

			// Asserting reset by checking if the replay is still primed or not
			let destination_2 = MockObserver::default();
			let notification_collector_2 = destination_2.get_notification_collector();
			let _s2 = connectable_observable.subscribe(destination_2);

			notification_collector_2.lock().assert_notifications(
				"connectable - reset_on_error 2",
				0,
				[
					SubscriberNotification::Error(error),
					SubscriberNotification::Unsubscribe,
				],
				true,
			);
		}
	}
}

mod disconnect_on_ref_count_zero {

	use super::*;

	mod when_enabled {
		use super::*;

		#[test]
		fn should_disconnect_when_the_subscriber_count_drops_to_zero() {
			let source = PublishSubject::<usize, &'static str>::default();
			let mut connectable_observable = ConnectableObservable::new(
				source.clone(),
				ConnectableOptions {
					connector_creator: PublishSubject::default,
					disconnect_when_ref_count_zero: true,
					reset_connector_on_disconnect: false,
					reset_connector_on_complete: false,
					reset_connector_on_error: false,
				},
			);

			connectable_observable.connect();
			let mut subscription_1 = connectable_observable.subscribe(MockObserver::default());
			let mut subscription_2 = connectable_observable.subscribe(MockObserver::default());
			assert!(connectable_observable.is_connected());

			subscription_1.unsubscribe();
			assert!(connectable_observable.is_connected());
			subscription_2.unsubscribe();

			assert!(
				!connectable_observable.is_connected(),
				"should've disconnected when the ref count dropped to 0"
			);
		}

		mod reset_on_disconnect {

			use super::*;

			mod when_enabled {
				use super::*;

				#[test]
				fn should_reset_when_disconnected_by_ref_count_zero() {
					let destination = MockObserver::default();
					let notification_collector = destination.get_notification_collector();

					let mut source = PublishSubject::<usize, &'static str>::default();
					// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
					// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
					let mut connectable_observable = ConnectableObservable::new(
						source.clone(),
						ConnectableOptions {
							connector_creator: ReplaySubject::<1, _, _>::default,
							disconnect_when_ref_count_zero: true,
							reset_connector_on_disconnect: true,
							reset_connector_on_complete: false,
							reset_connector_on_error: false,
						},
					);

					connectable_observable.connect();
					let mut subscription = connectable_observable.subscribe(destination);
					source.next(1);
					assert!(connectable_observable.is_connected());
					subscription.unsubscribe();
					assert!(
						!connectable_observable.is_connected(),
						"should've disconnected!"
					);

					notification_collector.lock().assert_notifications(
						"connectable - disconnect_and_reset_on_ref_zero 1",
						0,
						[
							SubscriberNotification::Next(1),
							SubscriberNotification::Unsubscribe,
						],
						true,
					);

					// Asserting reset by checking if the replay is still primed or not
					let destination_2 = MockObserver::default();
					let notification_collector_2 = destination_2.get_notification_collector();
					let _s2 = connectable_observable.subscribe(destination_2);
					notification_collector_2
						.lock()
						.assert_is_empty("connectable - disconnect_and_reset_on_ref_zero 2");
				}
			}

			mod when_disabled {
				use super::*;

				#[test]
				fn should_not_reset_when_disconnected_by_ref_count_zero() {
					let destination = MockObserver::default();
					let notification_collector = destination.get_notification_collector();

					let mut source = PublishSubject::<usize, &'static str>::default();
					// let mut connector_creator_call_count = Arc::new(AtomicUsize::new(0));
					// connector_creator_call_count.fetch_add(1, Ordering::Relaxed);
					let mut connectable_observable = ConnectableObservable::new(
						source.clone(),
						ConnectableOptions {
							connector_creator: ReplaySubject::<1, _, _>::default,
							disconnect_when_ref_count_zero: true,
							reset_connector_on_disconnect: false,
							reset_connector_on_complete: false,
							reset_connector_on_error: false,
						},
					);

					connectable_observable.connect();
					let mut subscription = connectable_observable.subscribe(destination);
					source.next(1);
					assert!(connectable_observable.is_connected());
					subscription.unsubscribe();
					assert!(!connectable_observable.is_connected());

					notification_collector.lock().assert_notifications(
						"connectable - disconnect_and_not_reset_on_ref_zero 1",
						0,
						[
							SubscriberNotification::Next(1),
							SubscriberNotification::Unsubscribe,
						],
						true,
					);

					// Asserting reset by checking if the replay is still primed or not
					let destination_2 = MockObserver::default();
					let notification_collector_2 = destination_2.get_notification_collector();
					let _s2 = connectable_observable.subscribe(destination_2);

					notification_collector_2.lock().assert_notifications(
						"connectable - disconnect_and_not_reset_on_ref_zero 2",
						0,
						[SubscriberNotification::Next(1)],
						true,
					);
				}
			}
		}
	}

	mod when_disabled {
		use super::*;

		#[test]
		fn should_not_disconnect_when_the_subscriber_count_drops_to_zero() {
			let source = PublishSubject::<usize, &'static str>::default();
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

			connectable_observable.connect();
			let mut subscription_1 = connectable_observable.subscribe(MockObserver::default());
			let mut subscription_2 = connectable_observable.subscribe(MockObserver::default());
			assert!(connectable_observable.is_connected());

			subscription_1.unsubscribe();
			assert!(connectable_observable.is_connected());
			subscription_2.unsubscribe();

			assert!(
				connectable_observable.is_connected(),
				"should stay connected if disconnect_when_ref_count_zero is disabled"
			);
		}
	}
}
