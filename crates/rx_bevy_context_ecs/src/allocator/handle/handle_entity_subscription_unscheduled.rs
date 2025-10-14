use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike, Teardown,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::UnscheduledSubscriptionHandle,
	},
};

use super::WeakEntitySubscriptionHandle;

pub struct UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	subscription_entity: Entity,
	_phantom_data: PhantomData<Subscription>,
}

impl<Subscription> UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<Subscription> UnscheduledSubscriptionHandle
	for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	type WeakHandle = WeakEntitySubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakEntitySubscriptionHandle::new(self.subscription_entity)
	}
}

impl<Subscription> WithSubscriptionContext for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Clone for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription_entity: self.subscription_entity.clone(),
			_phantom_data: PhantomData,
		}
	}
}

impl<Subscription> SubscriptionLike for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
	}
}

impl<Subscription> Drop for UnscheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
