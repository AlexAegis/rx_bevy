use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext, Mutable, StorageType},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Query, StaticSystemParam},
	world::DeferredWorld,
};
use rx_core_traits::{Observable, SubscriptionLike};
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	EntitySubscriptionContextAccessProvider, SubscriptionNotificationEvent,
	SubscriptionNotificationEventError,
};

#[derive(Component)]
#[component(on_insert=subscription_on_insert::<O, ContextAccess>, on_remove=subscription_on_remove::<O, ContextAccess>)]
#[require(Name::new(format!("Subscription ({})", short_type_name::<O>())))]
pub struct SubscriptionComponent<O, ContextAccess>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	subscription: O::Subscription,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<O, ContextAccess> SubscriptionComponent<O, ContextAccess>
where
	O: Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	pub(crate) fn new(subscription: O::Subscription) -> Self {
		Self {
			subscription,
			_phantom_data: PhantomData,
		}
	}
}

fn subscription_notification_observer<O, ContextAccess>(
	subscription_notification: Trigger<SubscriptionNotificationEvent<ContextAccess>>,
	mut subscription_query: Query<&mut SubscriptionComponent<O, ContextAccess>>,
	mut context: StaticSystemParam<BevySubscriptionContext<ContextAccess>>,
) -> Result<(), BevyError>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	let subscription_entity = subscription_notification.target();
	let Ok(mut subscription_component) = subscription_query.get_mut(subscription_entity) else {
		return Err(SubscriptionNotificationEventError::NotASubscription(
			short_type_name::<O>(),
			subscription_entity,
		)
		.into());
	};

	subscription_component
		.subscription
		.unsubscribe(&mut context);

	Ok(())
}

fn subscription_on_insert<O, ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		subscription_notification_observer::<O, ContextAccess>,
	));
}

fn subscription_on_remove<O, ContextAccess>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	O: 'static + Observable<Context = BevySubscriptionContextProvider<ContextAccess>> + Send + Sync,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	deferred_world.commands().trigger_targets(
		SubscriptionNotificationEvent::<ContextAccess>::Unsubscribe,
		hook_context.entity,
	);
}
