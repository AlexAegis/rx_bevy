use std::marker::PhantomData;

use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	observer::Observer,
	world::DeferredWorld,
};
use rx_core_traits::{
	ObservableSubscription, SubscriptionContext, SubscriptionLike, SubscriptionNotification,
	Teardown, Tick, Tickable, WithSubscriptionContext,
	allocator::handle::ScheduledSubscriptionHandle,
};

use crate::{
	BevySubscriptionContextProvider, observable_subscription_notification_observer,
	subscription_unsubscribe_on_remove,
};

use super::{UnscheduledEntitySubscriptionHandle, WeakEntitySubscriptionHandle};

#[derive(Component)]
#[component(on_insert=observable_subscription_add_notification_observer_on_insert::<Subscription>, on_remove=subscription_unsubscribe_on_remove::<Subscription>)]
pub struct ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	subscription_entity: Entity,
	closed: bool,
	_phantom_data: PhantomData<Subscription>,
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

impl<Subscription> ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

impl<Subscription> ScheduledSubscriptionHandle for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type WeakHandle = WeakEntitySubscriptionHandle<Subscription>;
	type UnscheduledHandle = UnscheduledEntitySubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakEntitySubscriptionHandle::new(self.subscription_entity)
	}

	fn clone(&self) -> Self::UnscheduledHandle {
		UnscheduledEntitySubscriptionHandle::new(self.subscription_entity)
	}
}

impl<Subscription> WithSubscriptionContext for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Tickable for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Tick(tick),
		);
	}
}

impl<Subscription> SubscriptionLike for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.closed = true;
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Add(teardown),
		);
	}
}

impl<Subscription> Drop for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + ObservableSubscription<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
