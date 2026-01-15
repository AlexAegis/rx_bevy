use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	world::DeferredWorld,
};
use rx_core_common::{SharedSubscription, SubscriptionLike, Teardown, TeardownCollectionExtension};
use rx_core_macro_subscription_derive::RxSubscription;

/// Allows attaching additional teardown to an entity when it despawns.
#[derive(Component, Deref)]
#[component(on_remove=subscription_satellite_of_on_remove)]
#[relationship(relationship_target=SubscriptionSatellites)]
#[require(SubscriptionSatellite)]
pub struct SubscriptionSatelliteOf {
	#[relationship]
	#[deref]
	entity: Entity,
}

impl SubscriptionSatelliteOf {
	pub fn new(entity: Entity, teardown: impl Into<Teardown>) -> (Self, SubscriptionSatellite) {
		(Self { entity }, SubscriptionSatellite::new(teardown))
	}
}

fn subscription_satellite_of_on_remove(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let subscription_satellite =
		deferred_world.get_mut::<SubscriptionSatellite>(hook_context.entity);

	subscription_satellite
		.expect("required component to exist")
		.unsubscribe();

	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<SubscriptionSatellite>();
}

#[derive(Component, Deref)]
#[relationship_target(relationship=SubscriptionSatelliteOf)]
pub struct SubscriptionSatellites {
	#[relationship]
	#[deref]
	satellites: Vec<Entity>,
}

#[derive(RxSubscription, Component, Deref, DerefMut, Default)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
pub struct SubscriptionSatellite {
	#[destination]
	#[deref]
	subscription: SharedSubscription,
}

impl SubscriptionSatellite {
	pub fn new(teardown: impl Into<Teardown>) -> Self {
		let mut subscription = SharedSubscription::default();
		subscription.add(teardown);
		Self { subscription }
	}
}
