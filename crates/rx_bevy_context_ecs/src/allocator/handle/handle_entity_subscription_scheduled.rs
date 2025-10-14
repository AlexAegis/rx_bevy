use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike, Teardown, Tick, Tickable,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::ScheduledSubscriptionHandle,
	},
};

use super::{UnscheduledEntitySubscriptionHandle, WeakEntitySubscriptionHandle};

pub struct ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	subscription_entity: Entity,
	_phantom_data: PhantomData<Subscription>,
}

impl<Subscription> ScheduledEntitySubscriptionHandle<Subscription>
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

impl<Subscription> ScheduledSubscriptionHandle for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
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
	Subscription: SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Tickable for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		todo!("impl")
	}
}

impl<Subscription> SubscriptionLike for ScheduledEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn is_closed(&self) -> bool {
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		todo!("impl")
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		todo!("impl")
	}
}

impl<Subscription> Drop for ScheduledEntitySubscriptionHandle<Subscription>
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
