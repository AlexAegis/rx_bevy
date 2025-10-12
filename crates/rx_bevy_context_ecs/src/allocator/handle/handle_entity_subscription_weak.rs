use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike, Teardown,
	context::{WithSubscriptionContext, allocator::handle::WeakSubscriptionHandle},
};

pub struct WeakEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	subscription_entity: Entity,
	_phantom_data: PhantomData<Subscription>,
}

impl<Subscription> WeakEntitySubscriptionHandle<Subscription>
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

impl<Subscription> WeakSubscriptionHandle for WeakEntitySubscriptionHandle<Subscription> where
	Subscription: SubscriptionLike + Send + Sync
{
}

impl<Subscription> WithSubscriptionContext for WeakEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Clone for WeakEntitySubscriptionHandle<Subscription>
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

impl<Subscription> SubscriptionLike for WeakEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn is_closed(&self) -> bool {
		todo!("impl")
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		todo!("impl")
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		todo!("impl")
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		todo!("impl")
	}
}

impl<Subscription> Drop for WeakEntitySubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		// Does not own its subscription so it must not do anything with it on drop.
		// It's not like it could from here anyway, but at least we
		// won't need to panic because we dropped an active subscription.

		// The component implementation of this handle must also not unsubscribe `on_remove`.
	}
}
