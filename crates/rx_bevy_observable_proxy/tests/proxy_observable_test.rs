use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

mod when_used_as_a_component {
	use super::*;

	#[test]
	fn should_observe_an_observable_entity() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn((1..=3_usize).into_observable().into_component())
			.id();

		let proxy_observable_entity = app
			.world_mut()
			.commands()
			.spawn(
				ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone())
					.into_component(),
			)
			.id();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(proxy_observable_entity)
			.as_observable::<usize, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
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
	fn should_observe_key_presses_and_emit_them_as_signals() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn((1..=3_usize).into_observable().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone());

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut subject = PublishSubject::<usize>::default();

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(subject.clone().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone());

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		subject.next(1);
		subject.complete();

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
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
				"proxy_observable - rx_verify_no_new_notification_after_closed",
				1,
			);
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut subject = PublishSubject::<usize, TestError>::default();

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(subject.clone().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, TestError>::new(observable_entity, scheduler_handle.clone());

		let destination = MockObserver::<usize, TestError>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		subject.next(1);
		subject.error(TestError);

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Error(TestError),
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
				"proxy_observable - rx_verify_no_new_notification_after_closed",
				1,
			);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut subject = PublishSubject::<usize>::default();

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(subject.clone().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone());

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		subject.next(1);

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
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
				"proxy_observable - rx_verify_no_new_notification_after_closed",
				1,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut subject = PublishSubject::<usize>::default();

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(subject.clone().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone())
				.take(1);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		subject.next(1);

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
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
				"proxy_observable - rx_verify_no_new_notification_after_closed",
				1,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let subject = PublishSubject::<usize>::default();

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(subject.clone().into_component())
			.id();

		let mut proxy_observable =
			ProxyObservable::<usize, Never>::new(observable_entity, scheduler_handle.clone())
				.take(0);

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = proxy_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("proxy_observable");

		app.update();

		notification_collector.lock().assert_notifications(
			"proxy_observable",
			0,
			[SubscriberNotification::Complete],
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
				"proxy_observable - rx_verify_no_new_notification_after_closed",
				0,
			);
	}
}
