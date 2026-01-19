use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_common::{SharedSubscription, Teardown};
use rx_core_testing::TrackTeardownExtension;

#[test]
fn subscription_satellite_unsubscribes_when_relationship_removed() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let host_entity = app.world_mut().spawn_empty().id();

	let subscription = SharedSubscription::default();
	let subscription_clone = subscription.clone();

	let satellite_entity = app
		.world_mut()
		.spawn(SubscriptionSatellite::new(host_entity, subscription))
		.id();

	app.update();

	app.world_mut()
		.entity_mut(satellite_entity)
		.remove::<SubscriptionSatelliteOf>();

	app.update();

	assert!(
		subscription_clone.is_closed(),
		"subscription should close when the satellite relationship is removed",
	);

	assert!(
		app.world().entities().contains(satellite_entity),
		"satellite entity should remain after removing only the relationship component",
	);

	assert!(
		app.world()
			.get::<SubscriptionComponent>(satellite_entity)
			.is_none(),
		"subscription component should be removed by the on_remove hook",
	);
}

#[test]
fn subscription_satellite_executes_teardown_when_despawned() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let host_entity = app.world_mut().spawn_empty().id();

	let (teardown, teardown_tracker) = Teardown::tracked("subscription_sattelite");

	let satellite_entity = app
		.world_mut()
		.spawn(SubscriptionSatellite::new_with_teardown(
			host_entity,
			teardown,
		))
		.id();

	app.update();

	app.world_mut()
		.commands()
		.entity(satellite_entity)
		.despawn();

	app.update();

	teardown_tracker.assert_was_torn_down();
}
