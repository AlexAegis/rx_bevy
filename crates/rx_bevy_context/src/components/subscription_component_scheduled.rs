use bevy_ecs::{
	component::{Component, HookContext},
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Query, StaticSystemParam},
	world::DeferredWorld,
};
use rx_core_traits::{
	ObservableSubscription, SubscriptionLike, Teardown, Tick, Tickable,
	SubscriptionContext, WithSubscriptionContext,
};
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	ConsumableSubscriptionNotificationEvent, SubscriptionNotificationEvent,
	SubscriptionNotificationEventError,
};

#[derive(Component)]
#[component(on_insert=observable_subscription_add_notification_observer_on_insert::<Subscription>, on_remove=subscription_unsubscribe_on_remove::<Subscription>)]
#[require(Name::new(short_type_name::<Self>()))]
pub struct ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	subscription: Subscription,
}

pub(crate) fn observable_subscription_add_notification_observer_on_insert<Subscription>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		observable_subscription_notification_observer::<Subscription>,
	));
}

pub(crate) fn observable_subscription_notification_observer<Subscription>(
	mut subscription_notification: Trigger<ConsumableSubscriptionNotificationEvent>,
	mut subscription_query: Query<&mut ScheduledSubscriptionComponent<Subscription>>,
	mut context: StaticSystemParam<BevySubscriptionContext>,
) -> Result<(), BevyError>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let subscription_entity = subscription_notification.target();
	let Ok(mut subscription_component) = subscription_query.get_mut(subscription_entity) else {
		return Err(SubscriptionNotificationEventError::NotASubscription(
			short_type_name::<Subscription>(),
			subscription_entity,
		)
		.into());
	};

	let event = subscription_notification
		.event_mut()
		.take()
		.expect("notification was already consumed!");

	let subscription = &mut subscription_component.subscription;
	match event {
		SubscriptionNotificationEvent::Unsubscribe => subscription.unsubscribe(&mut context),
		SubscriptionNotificationEvent::Tick(tick) => subscription.tick(tick, &mut context),
		SubscriptionNotificationEvent::Add(teardown) => {
			subscription.add_teardown(teardown, &mut context)
		}
	};

	Ok(())
}

pub(crate) fn subscription_unsubscribe_on_remove<Subscription>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	deferred_world.commands().trigger_targets(
		SubscriptionNotificationEvent::Unsubscribe,
		hook_context.entity,
	);
}

impl<Subscription> ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub(crate) fn new(subscription: Subscription) -> Self {
		Self { subscription }
	}
}

impl<Subscription> WithSubscriptionContext for ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Subscription> Tickable for ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subscription.tick(tick, context);
	}
}

impl<Subscription> SubscriptionLike for ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.subscription.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subscription.add_teardown(teardown, context);
	}
}
