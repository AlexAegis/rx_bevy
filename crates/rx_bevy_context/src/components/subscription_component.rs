use bevy_ecs::{
	component::{Component, HookContext},
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Query, StaticSystemParam},
	world::DeferredWorld,
};
use rx_core_traits::{Observable, SubscriptionLike};
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, SubscriptionNotificationEvent,
	SubscriptionNotificationEventError,
};

#[derive(Component)]
#[component(on_insert=subscription_on_insert::<O>, on_remove=subscription_on_remove::<O>)]
#[require(Name::new(format!("Subscription ({})", short_type_name::<O>())))]
pub struct SubscriptionComponent<O>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	subscription: O::Subscription,
}

impl<O> SubscriptionComponent<O>
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub(crate) fn new(subscription: O::Subscription) -> Self {
		Self { subscription }
	}
}

fn subscription_notification_observer<O>(
	subscription_notification: Trigger<SubscriptionNotificationEvent>,
	mut subscription_query: Query<&mut SubscriptionComponent<O>>,
	mut context: StaticSystemParam<BevySubscriptionContext>,
) -> Result<(), BevyError>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
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

fn subscription_on_insert<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(subscription_notification_observer::<O>));
}

fn subscription_on_remove<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	deferred_world.commands().trigger_targets(
		SubscriptionNotificationEvent::Unsubscribe,
		hook_context.entity,
	);
}
