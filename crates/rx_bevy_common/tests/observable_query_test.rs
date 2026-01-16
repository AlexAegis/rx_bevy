use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::{Never, SubscriberNotification};
use rx_core_testing::{MockObserver, NotificationCollector};

#[test]
fn observable_query_returns_error_when_entity_is_not_an_observable() {
	let mut app = App::new();

	let target_entity = app.world_mut().spawn_empty().id();

	let mut system_state =
		SystemState::<ObservableQuery<'_, '_, Never, Never>>::new(app.world_mut());
	let mut observable_query = system_state.get_mut(app.world_mut());

	let result = observable_query.try_subscribe_to(
		target_entity,
		MockObserver::new(NotificationCollector::default()),
	);

	system_state.apply(app.world_mut());

	match result {
		Err(SubscribeError::NotAnObservable(_, entity)) => {
			assert_eq!(entity, target_entity);
		}
		other => panic!("expected NotAnObservable error, got {other:?}"),
	}
}

mod using_subject_component {
	use super::*;

	#[test]
	fn observable_query_allows_for_checked_subscriptions() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let destination = MockObserver::<usize>::default();
		let notifications = destination.get_notification_collector();

		let mut subject = PublishSubject::<usize>::default();
		let observable_entity = app.world_mut().spawn(subject.clone().into_component()).id();

		// Run once so observable insertion hooks (outputs, satellites) are applied
		app.update();

		let mut system_state =
			SystemState::<ObservableQuery<'_, '_, usize, Never>>::new(app.world_mut());
		let mut observable_query = system_state.get_mut(app.world_mut());

		let subscription_entity = observable_query
			.try_subscribe_to(observable_entity, destination)
			.expect("subscription should succeed for matching observable outputs");

		system_state.apply(app.world_mut());
		app.update();

		assert!(
			app.world().get_entity(subscription_entity).is_ok(),
			"subscription entity should be alive after a successful subscription",
		);
		assert!(
			app.world()
				.get_entity(subscription_entity)
				.map(|entity| !entity.contains::<UnfinishedSubscription>())
				.unwrap_or(false),
			"unfinished marker should be removed after subscription completes",
		);

		subject.next(10);
		app.update();

		notifications.lock().assert_notifications(
			"observable_query - subject",
			0,
			[SubscriberNotification::Next(10)],
			true,
		);
	}
}

mod using_observable_component {
	use super::*;

	#[test]
	fn observable_query_handles_immediately_completing_observable() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let destination = MockObserver::<usize>::default();
		let notifications = destination.get_notification_collector();

		let observable_entity = app
			.world_mut()
			.spawn(OfObservable::new(7_usize).into_component())
			.id();

		app.update();

		let mut system_state =
			SystemState::<ObservableQuery<'_, '_, usize, Never>>::new(app.world_mut());
		let mut observable_query = system_state.get_mut(app.world_mut());

		let subscription_entity = observable_query
			.try_subscribe_to(observable_entity, destination)
			.expect("subscription should succeed for matching observable outputs");

		system_state.apply(app.world_mut());
		app.update();

		assert!(
			app.world().get_entity(subscription_entity).is_err(),
			"subscription entity should be despawned for immediately completing observables",
		);

		notifications.lock().assert_notifications(
			"observable_query - of observable",
			0,
			[
				SubscriberNotification::Next(7),
				SubscriberNotification::Complete,
			],
			true,
		);
	}
}
