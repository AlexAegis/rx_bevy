use bevy_ecs::{
	component::{Component, HookContext},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use disqualified::ShortName;
use rx_core_traits::{
	SubscriptionLike, SubscriptionNotification, SubscriptionWithTeardown, Teardown,
	TeardownCollection, WithSubscriptionContext,
};
use stealcell::{StealCell, Stolen};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	SubscriptionNotificationEvent,
};

#[derive(Component)]
#[component(on_insert=unscheduled_subscription_add_notification_observer_on_insert::<Subscription>, on_remove=unscheduled_subscription_unsubscribe_on_remove::<Subscription>)]
#[require(Name::new(format!("{}", ShortName::of::<Subscription>())))]
pub struct UnscheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	this_entity: Entity,
	subscription: StealCell<Subscription>,
}

fn unscheduled_subscription_unsubscribe_on_remove<Subscription>(
	deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let context_param: BevySubscriptionContextParam = deferred_world.into();
	let mut context = context_param.into_context(Some(hook_context.entity));

	let mut stolen_subscription = context
		.steal_unscheduled_subscription::<Subscription>(hook_context.entity)
		.unwrap();
	stolen_subscription.unsubscribe(&mut context);
	context
		.return_stolen_unscheduled_subscription(hook_context.entity, stolen_subscription)
		.unwrap();
}

impl<Subscription> UnscheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(subscription: Subscription, this_entity: Entity) -> Self {
		Self {
			subscription: StealCell::new(subscription),
			this_entity,
		}
	}

	fn get_subscription(&self) -> &Subscription {
		self.subscription.get()
	}

	fn get_subscription_mut(&mut self) -> &mut Subscription {
		self.subscription.get_mut()
	}

	pub fn steal_subscription(&mut self) -> Stolen<Subscription> {
		self.subscription.steal()
	}

	pub fn return_stolen_subscription(&mut self, subscription: Stolen<Subscription>) {
		self.subscription.return_stolen(subscription);
	}
}

fn unscheduled_subscription_add_notification_observer_on_insert<Subscription>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		unscheduled_subscription_notification_observer::<Subscription>,
	));
}

fn unscheduled_subscription_notification_observer<Subscription>(
	mut subscription_notification: Trigger<SubscriptionNotificationEvent>,
	context_param: BevySubscriptionContextParam,
) -> Result<(), BevyError>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let subscription_entity = subscription_notification.entity();
	let mut context = context_param.into_context(Some(subscription_entity));

	let mut stolen_subscription =
		context.steal_unscheduled_subscription::<Subscription>(subscription_entity)?;

	let event = subscription_notification.event_mut();

	match event.consume()? {
		SubscriptionNotification::Unsubscribe => stolen_subscription.unsubscribe(&mut context),
		SubscriptionNotification::Tick(_tick) => {} // These subscriptions are non-tickable, so this event is ignored
		SubscriptionNotification::Add(Some(teardown)) => {
			stolen_subscription.add_teardown(teardown, &mut context)
		}
		SubscriptionNotification::Add(None) => {}
	};

	context.return_stolen_unscheduled_subscription(subscription_entity, stolen_subscription)?;

	Ok(())
}

impl<Subscription> WithSubscriptionContext for UnscheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Subscription> SubscriptionLike for UnscheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.get_subscription().is_closed()
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.get_subscription_mut().unsubscribe(context);
			context
				.deferred_world
				.commands()
				.entity(self.this_entity)
				.try_despawn();
		}
	}
}

impl<Subscription> TeardownCollection for UnscheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + SubscriptionWithTeardown<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		self.get_subscription_mut().add_teardown(teardown, context);
	}
}
