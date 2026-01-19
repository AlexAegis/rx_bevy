use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::SharedSubscription;

#[test]
fn subscription_component_new_unsubscribes_when_entity_despawns() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let subscription = SharedSubscription::default();
	let subscription_clone = subscription.clone();

	let entity = app
		.world_mut()
		.spawn(SubscriptionComponent::new(subscription))
		.id();

	app.update();

	app.world_mut().commands().entity(entity).despawn();

	app.update();

	assert!(
		subscription_clone.is_closed(),
		"subscription should be unsubscribed when its entity is despawned"
	);
}

mod new_despawn_on_unsubscribe {
	use super::*;

	#[test]
	fn subscription_component_new_despawn_on_unsubscribe_despawns_entity_when_subscription_closes()
	{
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let subscription = SharedSubscription::default();
		let subscription_clone = subscription.clone();

		let entity = app.world_mut().spawn_empty().id();
		app.world_mut().entity_mut(entity).insert(
			SubscriptionComponent::new_despawn_on_unsubscribe(
				subscription,
				entity,
				scheduler_handle.clone(),
			),
		);

		app.update();

		// Unsubscribe from a clone of the internal SharedSubscription
		let mut subscription_clone = subscription_clone;
		subscription_clone.unsubscribe();

		app.update();

		assert!(
			!app.world().entities().contains(entity),
			"entity should be despawned when subscription unsubscribes"
		);
	}
}
