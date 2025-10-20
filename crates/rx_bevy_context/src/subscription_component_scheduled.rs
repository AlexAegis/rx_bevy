use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use rx_core_traits::{
	ObservableSubscription, SubscriptionLike, Teardown, Tick, Tickable, WithSubscriptionContext,
};
use short_type_name::short_type_name;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ConsumableSubscriptionNotificationEvent, SubscriptionNotificationEvent,
};

#[derive(Component)]
#[component(on_insert=scheduled_subscription_add_notification_observer_on_insert::<Subscription>, on_remove=scheduled_subscription_unsubscribe_on_remove::<Subscription>)]
#[require(Name::new(short_type_name::<Self>()))]
pub struct ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	this_entity: Entity,
	/// Stealable!
	subscription: Option<Subscription>,
}

impl<Subscription> ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(subscription: Subscription, this_entity: Entity) -> Self {
		Self {
			subscription: Some(subscription),
			this_entity,
		}
	}

	fn get_subscription(&self) -> &Subscription {
		self.subscription.as_ref().expect("Subscription is stolen!")
	}

	fn get_subscription_mut(&mut self) -> &mut Subscription {
		self.subscription.as_mut().expect("Subscription is stolen!")
	}

	pub fn steal_subscription(&mut self) -> Subscription {
		self.subscription
			.take()
			.expect("Subscription was already stolen!")
	}

	pub fn return_stolen_subscription(&mut self, subscription: Subscription) {
		if self.subscription.replace(subscription).is_some() {
			panic!("An subscription was returned but it wasn't stolen from here!")
		}
	}
}

pub(crate) fn scheduled_subscription_add_notification_observer_on_insert<Subscription>(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		scheduled_subscription_notification_observer::<Subscription>,
	));
}

pub(crate) fn scheduled_subscription_notification_observer<Subscription>(
	mut subscription_notification: Trigger<ConsumableSubscriptionNotificationEvent>,
	context_param: BevySubscriptionContextParam,
) -> Result<(), BevyError>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let subscription_entity = subscription_notification.target();
	let mut context = context_param.into_context(subscription_entity);
	let mut stolen_scheduled_subscription =
		context.steal_scheduled_subscription::<Subscription>(subscription_entity)?;

	let event = subscription_notification.event_mut().consume();

	match event {
		SubscriptionNotificationEvent::Unsubscribe => {
			stolen_scheduled_subscription.unsubscribe(&mut context);
			context
				.deferred_world
				.commands()
				.entity(subscription_entity)
				.despawn();
		}
		SubscriptionNotificationEvent::Tick(tick) => {
			stolen_scheduled_subscription.tick(tick, &mut context);
		}
		SubscriptionNotificationEvent::Add(teardown) => {
			stolen_scheduled_subscription.add_teardown(teardown, &mut context);
		}
	};

	context.return_stolen_scheduled_subscription::<Subscription>(
		subscription_entity,
		stolen_scheduled_subscription,
	)?;

	Ok(())
}

fn scheduled_subscription_unsubscribe_on_remove<Subscription>(
	deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let context_param: BevySubscriptionContextParam = deferred_world.into();
	let mut context = context_param.into_context(hook_context.entity);

	println!("subscription, unsub on remove?? {}", hook_context.entity);
	let mut stolen_subscription = context
		.steal_scheduled_subscription::<Subscription>(hook_context.entity)
		.unwrap();
	stolen_subscription.unsubscribe(&mut context);
	context
		.return_stolen_scheduled_subscription(hook_context.entity, stolen_subscription)
		.unwrap();
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
	#[inline]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		let subscription = self.get_subscription_mut();
		subscription.tick(tick, context);
	}
}

impl<Subscription> SubscriptionLike for ScheduledSubscriptionComponent<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		let subscription = self.get_subscription();
		subscription.is_closed()
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		println!("UNSUB SCHEDULED HANDLE");
		let subscription = self.get_subscription_mut();
		subscription.unsubscribe(context);
		context
			.deferred_world
			.commands()
			.entity(self.this_entity)
			.try_despawn();
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		let subscription = self.get_subscription_mut();
		subscription.add_teardown(teardown, context);
	}
}
