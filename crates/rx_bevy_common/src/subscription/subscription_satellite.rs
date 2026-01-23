use bevy_derive::Deref;
use bevy_ecs::{
	bundle::Bundle, component::Component, entity::Entity, lifecycle::HookContext, name::Name,
	world::DeferredWorld,
};
use rx_core_common::{SharedSubscription, SubscriptionLike, Teardown, TeardownCollectionExtension};

use crate::SubscriptionComponent;

/// # [SubscriptionSatellite]
///
/// Allows attaching additional teardowns to an entity to be executed when it
/// despawns.
#[derive(Bundle)]
pub struct SubscriptionSatellite {
	pub subscription_satellite_of: SubscriptionSatelliteOf,
	pub subscription_satellite: SubscriptionComponent,
}

impl SubscriptionSatellite {
	pub fn new(entity: Entity, subscription: SharedSubscription) -> Self {
		Self {
			subscription_satellite_of: SubscriptionSatelliteOf { entity },
			subscription_satellite: SubscriptionComponent::new(subscription),
		}
	}

	pub fn new_with_teardown(entity: Entity, teardown: impl Into<Teardown>) -> Self {
		let mut subscription_satellite = SubscriptionComponent::default();
		subscription_satellite.add(teardown.into());
		Self {
			subscription_satellite_of: SubscriptionSatelliteOf { entity },
			subscription_satellite,
		}
	}
}

#[derive(Component, Deref)]
#[component(on_remove=subscription_satellite_of_on_remove)]
#[relationship(relationship_target=SubscriptionSatellites)]
#[require(SubscriptionComponent, Name::new("Subscription Satellite"))]
pub struct SubscriptionSatelliteOf {
	#[relationship]
	#[deref]
	entity: Entity,
}

fn subscription_satellite_of_on_remove(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let subscription_satellite =
		deferred_world.get_mut::<SubscriptionComponent>(hook_context.entity);

	subscription_satellite
		.expect("required component to exist")
		.unsubscribe();

	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<SubscriptionComponent>();
}

#[derive(Component, Deref)]
#[relationship_target(relationship=SubscriptionSatelliteOf)]
pub struct SubscriptionSatellites {
	#[relationship]
	#[deref]
	satellites: Vec<Entity>,
}
