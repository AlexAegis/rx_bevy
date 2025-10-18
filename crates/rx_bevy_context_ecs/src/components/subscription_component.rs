use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	world::DeferredWorld,
};
use rx_bevy_core::Observable;

use crate::{BevySubscriptionContextProvider, EntitySubscriptionContextAccessProvider};

#[derive(Component)]
#[component(on_insert=subscription_on_insert::<ContextAccess>, on_remove=subscription_on_remove::<ContextAccess>)]
pub struct SubscriptionComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	observable_entity: Entity,
	subscription: O::Subscription,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<O, ContextAccess> SubscriptionComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	pub(crate) fn new(observable_entity: Entity, subscription: O::Subscription) -> Self {
		Self {
			observable_entity,
			subscription,
			_phantom_data: PhantomData,
		}
	}
}

fn subscription_on_insert<ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	// TODO: Insert (not spawn) Observer for Subscriptionnotifications
}

fn subscription_on_remove<ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	// TODO: Unsubscribe!
}
