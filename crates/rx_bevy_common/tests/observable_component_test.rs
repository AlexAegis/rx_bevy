use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

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
