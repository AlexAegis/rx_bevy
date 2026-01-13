use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Resource, Debug, PartialEq, Default)]
struct TestResource {
	pub value: usize,
}

mod when_used_as_a_component {
	use super::*;

	#[test]
	fn should_observe_changes() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(
				ResourceObservable::<TestResource, _, usize>::new(
					|test_resource: &TestResource| test_resource.value,
					ResourceObservableOptions {
						trigger_on_is_added: false,
						trigger_on_is_changed: true,
					},
					scheduler_handle.clone(),
				)
				.into_component(),
			)
			.id();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(observable_entity)
			.as_observable::<usize, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;
		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_observe_adding_resources_but_no_changes() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(
				ResourceObservable::<TestResource, _, usize>::new(
					|test_resource: &TestResource| test_resource.value,
					ResourceObservableOptions {
						trigger_on_is_added: true,
						trigger_on_is_changed: false,
					},
					scheduler_handle.clone(),
				)
				.into_component(),
			)
			.id();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(observable_entity)
			.as_observable::<usize, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_observe_changes_and_additions_both_by_default() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(
				ResourceObservable::<TestResource, _, usize>::new(
					|test_resource: &TestResource| test_resource.value,
					ResourceObservableOptions::default(),
					scheduler_handle.clone(),
				)
				.into_component(),
			)
			.id();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(observable_entity)
			.as_observable::<usize, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;
		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Unsubscribe,
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
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut resource_observable = ResourceObservable::<TestResource, _, usize>::new(
			|test_resource: &TestResource| test_resource.value,
			ResourceObservableOptions {
				trigger_on_is_added: false,
				trigger_on_is_changed: true,
			},
			scheduler_handle.clone(),
		);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = resource_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		subscription.unsubscribe();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_not_emit_again_for_repeated_updates_without_changing_the_resource() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut resource_observable = ResourceObservable::<TestResource, _, usize>::new(
			|test_resource: &TestResource| test_resource.value,
			ResourceObservableOptions {
				trigger_on_is_added: false,
				trigger_on_is_changed: true,
			},
			scheduler_handle.clone(),
		);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = resource_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();
		app.update();
		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();
		app.update();
		app.update();
		app.update();
		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		subscription.unsubscribe();

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update();

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
				"resource_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
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
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut resource_observable = ResourceObservable::<TestResource, _, usize>::new(
			|test_resource: &TestResource| test_resource.value,
			ResourceObservableOptions {
				trigger_on_is_added: false,
				trigger_on_is_changed: true,
			},
			scheduler_handle.clone(),
		);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = resource_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		subscription.unsubscribe();

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update();

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
				"resource_observable - rx_verify_no_new_notification_after_closed",
				3,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut resource_observable = ResourceObservable::<TestResource, _, usize>::new(
			|test_resource: &TestResource| test_resource.value,
			ResourceObservableOptions {
				trigger_on_is_added: false,
				trigger_on_is_changed: true,
			},
			scheduler_handle.clone(),
		)
		.take(2);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = resource_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		app.update();

		app.world_mut().resource_mut::<TestResource>().value += 1;

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

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
				"resource_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<TestResource>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut resource_observable = ResourceObservable::<TestResource, _, usize>::new(
			|test_resource: &TestResource| test_resource.value,
			ResourceObservableOptions {
				trigger_on_is_added: false,
				trigger_on_is_changed: true,
			},
			scheduler_handle.clone(),
		)
		.take(0);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = resource_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("resource_observable");

		notification_collector.lock().assert_notifications(
			"resource_observable",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

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
				"resource_observable - rx_verify_no_new_notification_after_closed",
				0,
			);
	}
}
