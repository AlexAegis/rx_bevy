use bevy_ecs::{
	component::{Component, HookContext},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use disqualified::ShortName;
use rx_core_common::{
	SchedulerHandle, SharedSubscription, SubscriptionLike, SubscriptionNotification,
	SubscriptionWithTeardown, Teardown, TeardownCollection,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::{
	RxBevyScheduler, RxBevySchedulerDespawnEntityExtension, SubscriptionNotificationEvent,
};

// TODO(bevy-0.18+): This component does not need to be erased, it's only erased to facilitate mass unsubscribe on exit, which currently can't be done using commands as there is no teardown schedule in bevy similar to the startup schedule. https://github.com/AlexAegis/rx_bevy/issues/2 https://github.com/bevyengine/bevy/issues/7067
#[derive(Component, RxSubscription, Clone)]
#[component(on_insert=subscription_add_notification_observer_on_insert, on_remove=subscription_unsubscribe_on_remove)]
#[require(Name::new(format!("{}", ShortName::of::<Self>())))]
pub struct SubscriptionComponent {
	this_entity: Entity,
	self_despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	subscription: SharedSubscription,
}

impl SubscriptionComponent {
	pub fn new<Subscription>(
		subscription: Subscription,
		this_entity: Entity,
		despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self
	where
		Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
	{
		Self {
			subscription: SharedSubscription::new(subscription),
			self_despawn_scheduler: despawn_scheduler,
			this_entity,
		}
	}
}

pub(crate) fn subscription_add_notification_observer_on_insert(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);

	entity_commands
		.insert(Observer::new(subscription_notification_observer).with_entity(hook_context.entity));
}

pub(crate) fn subscription_notification_observer(
	mut subscription_notification: Trigger<SubscriptionNotificationEvent>,
	mut deferred_world: DeferredWorld,
) -> Result<(), BevyError> {
	let subscription_entity = subscription_notification.entity();

	if let Some(mut subscription_component) =
		deferred_world.get_mut::<SubscriptionComponent>(subscription_entity)
	{
		match subscription_notification.event_mut().consume()? {
			SubscriptionNotification::Unsubscribe => {
				subscription_component.unsubscribe();
				deferred_world
					.commands()
					.entity(subscription_entity)
					.try_despawn();
			}
		};
	}

	Ok(())
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

impl SubscriptionLike for SubscriptionComponent {
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.subscription.is_closed() {
			self.subscription.unsubscribe();
		}

		self.self_despawn_scheduler
			.lock()
			.schedule_despawn_entity(self.this_entity, None);
	}
}

impl TeardownCollection for SubscriptionComponent {
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.subscription.add_teardown(teardown);
	}
}
