use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[path = "./utilities.rs"]
mod utilities;

use utilities::*;

mod given_a_subject_component {
	use super::*;

	mod when_sending_signals_directly_through_the_subject {
		use super::*;

		#[test]
		fn then_signals_should_reach_the_destination_entity() {
			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let notifications = NotificationCollector::default();

			let mut subject = PublishSubject::<usize>::default();
			let subject_entity = app.world_mut().spawn(subject.clone().into_component()).id();
			let destination_entity = app
				.world_mut()
				.spawn_empty()
				.observe(collect_notifications_into::<usize, Never>(
					notifications.clone(),
				))
				.id();

			let scheduler_handle = {
				let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
					.get_mut(app.world_mut());
				scheduler.handle()
			};

			let mut destination =
				EntityDestination::<usize>::new(destination_entity, scheduler_handle).upgrade();
			let tracked_teardown = destination.add_tracked_teardown("entity_destination");

			let subscription_entity = app
				.world_mut()
				.commands()
				.subscribe(subject_entity, destination);

			app.update();

			subject.next(1);
			subject.next(2);

			app.update();

			app.world_mut().despawn(subscription_entity); // Triggers an unsubscribe

			subject.next(99);

			app.update();

			notifications.lock().assert_notifications(
				"entity_destination_next",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Next(2),
				],
				true,
			);

			tracked_teardown.assert_was_torn_down();
		}
	}

	mod when_sending_signals_by_events_through_the_subject_entity {
		use super::*;

		#[test]
		fn then_signals_should_reach_the_destination_entity() {
			let mut app = App::new();
			app.init_resource::<Time<Virtual>>();
			app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

			let notifications = NotificationCollector::default();

			let subject_entity = app
				.world_mut()
				.spawn(PublishSubject::<usize>::default().into_component())
				.id();
			let destination_entity = app
				.world_mut()
				.spawn_empty()
				.observe(collect_notifications_into::<usize, Never>(
					notifications.clone(),
				))
				.id();

			let scheduler_handle = {
				let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
					.get_mut(app.world_mut());
				scheduler.handle()
			};

			let mut destination =
				EntityDestination::<usize>::new(destination_entity, scheduler_handle).upgrade();
			let tracked_teardown = destination.add_tracked_teardown("entity_destination");

			let subscription_entity = app
				.world_mut()
				.commands()
				.subscribe(subject_entity, destination);

			app.update();

			app.world_mut()
				.commands()
				.entity(subject_entity)
				.trigger(RxSignal::<usize, Never>::new_next(1, subject_entity));

			app.world_mut()
				.commands()
				.entity(subject_entity)
				.trigger(RxSignal::<usize, Never>::new_next(2, subject_entity));

			app.update();

			app.world_mut().despawn(subscription_entity); // Triggers an unsubscribe

			app.world_mut()
				.commands()
				.entity(subject_entity)
				.trigger(RxSignal::<usize, Never>::new_next(99, subject_entity));

			app.update();

			notifications.lock().assert_notifications(
				"entity_destination_next",
				0,
				[
					SubscriberNotification::Next(1),
					SubscriberNotification::Next(2),
				],
				true,
			);

			tracked_teardown.assert_was_torn_down();
		}
	}
}
