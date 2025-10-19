use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};
use rx_core_traits::{
	SubscriptionContext, SubscriptionLike, SubscriptionNotification, Teardown,
	WithSubscriptionContext, allocator::handle::UnscheduledSubscriptionHandle,
};

use crate::{
	BevySubscriptionContextProvider, subscription_unsubscribe_on_remove,
	unscheduled_subscription_add_notification_observer_on_insert,
};

use super::WeakEntitySubscriptionHandle;

#[derive(Component)]
#[component(on_insert=unscheduled_subscription_add_notification_observer_on_insert::<Subscription>, on_remove=subscription_unsubscribe_on_remove::<Subscription>)]
pub struct UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	subscription_entity: Entity,
	closed: bool,
	_phantom_data: PhantomData<Subscription>,
}

impl<Subscription> UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub(crate) fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

impl<Subscription> UnscheduledSubscriptionHandle
	for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type WeakHandle = WeakEntitySubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakEntitySubscriptionHandle::new(self.subscription_entity)
	}
}

impl<Subscription> WithSubscriptionContext for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Clone for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription_entity: self.subscription_entity,
			closed: self.closed,
			_phantom_data: PhantomData,
		}
	}
}

impl<Subscription> SubscriptionLike for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
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

impl<Subscription> Drop for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription:
		'static + SubscriptionLike<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
