use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

mod subscribe {
	use super::*;

	mod when_subscribe_succeeds {
		use super::*;

		#[test]
		fn should_be_able_to_subscribe_to_an_observable_component() {
			let destination = MockObserver::<usize>::default();
			let notification_collector = destination.get_notification_collector();

			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let mut subject = PublishSubject::<usize>::default();

			let observable_entity = app.world_mut().spawn(subject.clone().into_component()).id();

			let mut commands = app.world_mut().commands();
			subject.next(0);

			let subscription_entity = commands.subscribe(observable_entity, destination);
			app.update();

			assert!(
				app.world().get_entity(subscription_entity).is_ok(),
				"Subscription Entity should've been spawned!"
			);

			subject.next(1);

			app.world_mut()
				.commands()
				.entity(subscription_entity)
				.despawn();
			app.update();

			subject.next(2);

			notification_collector.lock().assert_notifications(
				"subject_component",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Unsubscribe,
				],
				true,
			);
		}

		#[test]
		fn should_despawn_reserved_subscription_when_no_observable_is_present() {
			let destination = MockObserver::<usize>::default();

			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let not_an_observable = app.world_mut().spawn_empty().id();

			let mut commands = app.world_mut().commands();
			let subscription_entity = commands.subscribe(not_an_observable, destination);

			app.update();

			assert!(
				app.world().get_entity(subscription_entity).is_err(),
				"unfinished subscription should be cleaned up in the same frame when it cannot attach",
			);

			assert!(
				app.world().get_entity(not_an_observable).is_ok(),
				"The not_an_observable entity should still exist!"
			);
		}

		#[test]
		fn should_not_spawn_a_subscription_entity_for_an_immediately_closed_subscription() {
			let destination = MockObserver::<usize>::default();
			let notification_collector = destination.get_notification_collector();

			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let observable_entity = app
				.world_mut()
				.spawn(OfObservable::new(1_usize).into_component())
				.id();

			let mut commands = app.world_mut().commands();

			let subscription_entity = commands.subscribe(observable_entity, destination);
			app.update();

			assert!(
				app.world().get_entity(subscription_entity).is_err(),
				"Subscription Entity should've not been spawned!"
			);

			notification_collector.lock().assert_notifications(
				"observable_component - immediate completion",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Complete,
				],
				true,
			);
		}
	}

	mod when_subscribe_fails {
		use super::*;

		#[test]
		fn it_should_automatically_despawn_the_reserved_subscription_entity() {
			let destination = MockObserver::<usize>::default();

			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let not_an_observable_entity = app.world_mut().spawn_empty().id();

			let mut commands = app.world_mut().commands();

			let subscription_entity = commands.subscribe(not_an_observable_entity, destination);

			for _i in 0..=SUBSCRIBE_COMMAND_MAX_RETRIES {
				app.update();
			}

			assert!(
				app.world().get_entity(subscription_entity).is_err(),
				"Entity should've been despawned!"
			);
		}
	}
}
mod component_remove {

	use super::*;

	/// Further subscriptions are impossible, but existing ones should not be
	/// affected when it's not affected.
	#[test]
	fn it_should_remove_other_required_components_when_removed_but_not_despawn_subscriptions() {
		let destination = MockObserver::<usize>::default();
		let notification_collector = destination.get_notification_collector();

		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.spawn(
				IntervalObservable::new(
					IntervalObservableOptions {
						duration: Duration::from_millis(1000),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler,
				)
				.into_component(),
			)
			.id();

		let mut commands = app.world_mut().commands();

		let _subscription_entity = commands.subscribe(observable_entity, destination);
		app.update();

		assert!(
			app.world()
				.get::<SubscribeObserverRef<IntervalObservable<RxBevyScheduler>>>(observable_entity)
				.is_some(),
			"Entity should have a SubscribeObserverRef component"
		);

		assert!(
			app.world()
				.get::<ObservableSubscriptions<IntervalObservable<RxBevyScheduler>>>(
					observable_entity
				)
				.is_some(),
			"Entity should have an ObservableSubscriptions component"
		);

		app.world_mut()
			.commands()
			.entity(observable_entity)
			.remove::<ObservableComponent<IntervalObservable<RxBevyScheduler>>>();

		app.update();

		assert!(
			app.world()
				.get::<SubscribeObserverRef<IntervalObservable<RxBevyScheduler>>>(observable_entity)
				.is_none(),
			"Entity should no longer have a SubscribeObserverRef component"
		);

		assert!(
			app.world()
				.get::<ObservableSubscriptions<IntervalObservable<RxBevyScheduler>>>(
					observable_entity
				)
				.is_none(),
			"Entity should no longer have an ObservableSubscriptions component"
		);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_secs(2));

		app.update();

		notification_collector.lock().assert_notifications(
			"observable_component - removed component",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Next(1),
			],
			true,
		);
	}

	#[test]
	fn it_should_not_unsubscribe_a_shared_observable_when_dropped_either() {
		let destination = MockObserver::<usize>::default();
		let notification_collector = destination.get_notification_collector();

		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.spawn(
				IntervalObservable::new(
					IntervalObservableOptions {
						duration: Duration::from_millis(1000),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler,
				)
				.share(ConnectableOptions::<
					ProvideWithDefault<PublishSubject<usize, Never>>,
				>::default())
				.into_component(),
			)
			.id();

		let mut commands = app.world_mut().commands();

		let subscription_entity = commands.subscribe(observable_entity, destination);

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_secs(2));

		app.update();

		notification_collector.lock().assert_notifications(
			"observable_component - shared interval",
			0,
			[
				SubscriberNotification::Next(0),
				SubscriberNotification::Next(1),
			],
			true,
		);

		app.world_mut()
			.commands()
			.entity(observable_entity)
			.despawn();

		app.update();

		app.world_mut()
			.resource_mut::<Time<Virtual>>()
			.advance_by(Duration::from_secs(2));

		app.update();

		app.world_mut()
			.commands()
			.entity(subscription_entity)
			.despawn();
		app.update();

		notification_collector.lock().assert_notifications(
			"observable_component - shared interval 2",
			2,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}
