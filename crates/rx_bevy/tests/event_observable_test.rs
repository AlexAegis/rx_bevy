use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[derive(EntityEvent, Clone, Debug, PartialEq)]
struct TestEvent {
	pub entity: Entity,
	pub value: usize,
}

mod when_used_as_a_component_that_observes_itself {
	use super::*;

	#[test]
	fn should_observe_bevy_events_and_emit_them_as_signals() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone())
				.into_component(),
		);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<TestEvent, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});

		subscription.unsubscribe();

		app.update(); // EntitySubscriptions unsubscribe by despawn

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 2,
		});

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_complete_when_the_observed_entity_despawns() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone())
				.into_component(),
		);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<TestEvent, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});

		app.update();

		app.world_mut().commands().entity(event_target).despawn();

		app.update();

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}

mod when_used_directly {
	use super::*;

	#[test]
	fn should_observe_bevy_events_and_emit_them_as_signals() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});

		subscription.unsubscribe();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 2,
		});

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_complete_when_the_observed_entity_despawns() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});
		app.world_mut().commands().entity(event_target).despawn();

		app.update();

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	mod when_the_observed_entity_is_missing_when_the_observer_spawns {
		use super::*;

		#[test]
		fn should_complete_when_despawned_before_the_subscription_work_runs() {
			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let scheduler_handle = {
				let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
					.get_mut(app.world_mut())
					.unwrap();
				scheduler.handle()
			};

			let event_target = app.world_mut().spawn_empty().id();
			app.update();
			let entity_count_before_subscribe = app.world().entity_count();

			let mut event_observable =
				EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

			let destination = MockObserver::<TestEvent, Never>::default();
			let notification_collector = destination.get_notification_collector();

			let mut subscription = event_observable.subscribe(destination);
			let tracked_teardown = subscription.add_tracked_teardown("event_observable");

			app.world_mut().despawn(event_target);

			app.update();

			notification_collector.lock().assert_notifications(
				"event_observable",
				0,
				[SubscriberNotification::Complete],
				true,
			);

			tracked_teardown.assert_was_torn_down();
			assert!(subscription.is_closed());

			app.update(); // Runs the invoked despawn of the observer satellite

			assert!(
				app.world()
					.resource::<RxBevyExecutor<Update, Virtual>>()
					.is_empty(),
				"No work should remain in the executor"
			);
			assert_eq!(
				app.world().entity_count(),
				entity_count_before_subscribe - 1,
				"Only the observed entity should be gone"
			);
		}

		#[test]
		fn should_complete_when_despawned_by_earlier_work_in_the_same_tick() {
			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let scheduler_handle = {
				let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
					.get_mut(app.world_mut())
					.unwrap();
				scheduler.handle()
			};

			let event_target = app.world_mut().spawn_empty().id();
			app.update();
			let entity_count_before_subscribe = app.world().entity_count();

			{
				let mut scheduler = scheduler_handle.lock();
				let cancellation_id = scheduler.generate_cancellation_id();
				scheduler.schedule_immediate_work(
					move |_, context| {
						context
							.deferred_world
							.commands()
							.entity(event_target)
							.despawn();
					},
					cancellation_id,
				);
			}

			let mut event_observable =
				EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

			let destination = MockObserver::<TestEvent, Never>::default();
			let notification_collector = destination.get_notification_collector();

			let mut subscription = event_observable.subscribe(destination);
			let tracked_teardown = subscription.add_tracked_teardown("event_observable");

			app.update();

			notification_collector.lock().assert_notifications(
				"event_observable",
				0,
				[SubscriberNotification::Complete],
				true,
			);

			tracked_teardown.assert_was_torn_down();
			assert!(subscription.is_closed());

			app.update(); // Runs the invoked despawn of the observer satellite

			assert!(
				app.world()
					.resource::<RxBevyExecutor<Update, Virtual>>()
					.is_empty(),
				"No work should remain in the executor"
			);
			assert_eq!(
				app.world().entity_count(),
				entity_count_before_subscribe - 1,
				"Only the observed entity should be gone"
			);
		}
	}
}

/// Non Applicable:
/// - rx_contract_closed_after_complete - Can't Complete
/// - rx_contract_closed_after_error - Can't Error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});

		subscription.unsubscribe();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 2,
		});

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update(); // To let the event observer_satellite_entity despawn

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"event_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone()).take(2);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 1,
		});
		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 2,
		});

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 0,
				}),
				SubscriberNotification::Next(TestEvent {
					entity: event_target,
					value: 1,
				}),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update(); // To let the event observer_satellite_entity despawn

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"event_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut())
				.unwrap();
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone()).take(0);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut().trigger(TestEvent {
			entity: event_target,
			value: 0,
		});
		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update(); // To let the event observer_satellite_entity despawn

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"event_observable - rx_verify_no_new_notification_after_closed",
				0,
			);
	}
}
