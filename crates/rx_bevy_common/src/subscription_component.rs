use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
	component::Component, entity::Entity, lifecycle::HookContext, name::Name, world::DeferredWorld,
};
use disqualified::ShortName;
use rx_core_common::{
	SchedulerHandle, SharedSubscription, SubscriptionLike, Teardown, TeardownCollectionExtension,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::{RxBevyScheduler, RxBevySchedulerDespawnEntityExtension};

#[derive(Component, RxSubscription, Clone, Deref, DerefMut, Default)]
#[component(on_remove=subscription_unsubscribe_on_remove)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
#[require(Name::new(format!("{}", ShortName::of::<Self>())))]
pub struct SubscriptionComponent {
	#[destination]
	#[deref]
	subscription: SharedSubscription,
}

impl SubscriptionComponent {
	/// A subscription component that:
	/// - Unsubscribes the internal subscription on remove
	/// - Does not despawn itself when the internal subscription unsubscribes,
	///   as it doesn't know its own entity
	pub fn new(subscription: SharedSubscription) -> Self {
		Self { subscription }
	}

	/// A subscription component that:
	/// - Unsubscribes the internal subscription on remove
	/// - Despawn an entity when the internal subscription
	///   unsubscribes. (Usually itself for when it's desired)
	pub fn new_despawn_on_unsubscribe(
		mut subscription: SharedSubscription,
		despawn_entity: Entity,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		subscription.add(Teardown::new(move || {
			despawn_scheduler
				.lock()
				.schedule_despawn_entity(despawn_entity, None);
		}));

		Self { subscription }
	}
}

fn subscription_unsubscribe_on_remove(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let mut subscription_component = deferred_world
		.get_mut::<SubscriptionComponent>(hook_context.entity)
		.unwrap();

	subscription_component.unsubscribe();
}
