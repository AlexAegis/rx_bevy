use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use bevy_ecs::{component::Component, entity::Entity};

use rx_bevy_core::{
	SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable, context::WithSubscriptionContext,
	prelude::SubscriptionContext,
};

use crate::BevySubscriberContext;

pub struct FooTeardown {
	// teardown_fn: Option<Box<dyn FnOnce(&mut Context) + Send + Sync>>,
}

#[derive(Component)]
pub struct EntitySubscription<'world, 'state> {
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	/// As this subscriber is stored in this entity!
	self_entity: Entity,
	subscription: SubscriptionData<BevySubscriberContext<'world, 'state>>,
	// sasd: NotifiableSubscription<Context>
	// finalizers: Vec<Box<fn(&Context)>>,
	//finalizers: Arc<RwLock<dyn FnOnce(&mut Context) + Send + Sync>>,
	//finalizers: Vec<Box<dyn FnOnce(&mut Context) + Send + Sync>>,
	//_phantom_data: PhantomData<fn(Context)>,
	// _phantom_data: PhantomData<fn(&'w mut Context) -> Context>,
}

impl<'world, 'state> EntitySubscription<'world, 'state> {
	pub fn new(self_entity: Entity) -> Self {
		Self {
			self_entity,
			subscription: SubscriptionData::default(),
			// finalizers: Vec::new(), // subscription: SubscriptionData::default(),
			// _phantom_data: PhantomData,
		}
	}

	pub fn new_with_teardown(
		self_entity: Entity,
		teardown: Teardown<BevySubscriberContext<'world, 'state>>,
	) -> Self {
		Self {
			self_entity,
			subscription: SubscriptionData::new_with_teardown(teardown),
		}
		// if let Some(took) = teardown.take() {
		// 	Self {
		// 		self_entity,
		// 		subscription: SubscriptionData::default(),
		// 		// finalizers: vec![took],
		// 		// _phantom_data: PhantomData,
		// 	}
		// } else {
		// 	Self::new(self_entity)
		// }
	}

	#[inline]
	pub fn get_self_entity(&self) -> Entity {
		self.self_entity
	}
}

impl<'world, 'state> WithSubscriptionContext for EntitySubscription<'world, 'state> {
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state> SubscriptionLike for EntitySubscription<'world, 'state> {
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.subscription.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subscription.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		BevySubscriberContext::<'world, 'state>::create_context_to_unsubscribe_on_drop()
	}
}

impl<'world, 'state> Tickable for EntitySubscription<'world, 'state> {
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.subscription.tick(tick, context);
	}
}

impl<'world, 'state> Drop for EntitySubscription<'world, 'state> {
	fn drop(&mut self) {
		// Only panics when the `dev_panic_on_dropped_active_subscriptions`
		// feature is active, otherwise it just prints a warning.

		if !self.is_closed() {
			let message = format!(
				"{} was dropped without unsubscribing first!",
				short_type_name::short_type_name::<Self>()
			);
			#[cfg(not(feature = "dev_panic_on_dropped_active_subscriptions"))]
			bevy_log::warn!("{}", message);

			#[cfg(feature = "dev_panic_on_dropped_active_subscriptions")]
			panic!("{}", message);
		}
	}
}
