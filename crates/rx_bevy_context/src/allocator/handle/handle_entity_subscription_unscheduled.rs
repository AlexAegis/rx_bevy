use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	entity::Entity,
	error::BevyError,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use rx_core_traits::{
	SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, SubscriptionNotification,
	Teardown, TeardownCollection, WithSubscriptionContext,
	allocator::handle::UnscheduledSubscriptionHandle,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	SubscriptionNotificationEvent, handle::ErasedEntitySubscriptionHandle,
};

use super::WeakEntitySubscriptionHandle;

#[derive(Component)]
#[component(on_insert=erased_subscription_add_notification_observer_on_insert, on_remove=erased_subscription_unsubscribe_on_remove::<Self>)]
pub struct UnscheduledEntitySubscriptionHandle {
	subscription_entity: Entity,
	closed_flag: SubscriptionClosedFlag,
}

pub(crate) fn erased_subscription_add_notification_observer_on_insert(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(erased_subscription_notification_observer));
}

fn erased_subscription_notification_observer(
	mut subscription_notification: Trigger<SubscriptionNotificationEvent>,
	context_param: BevySubscriptionContextParam,
) -> Result<(), BevyError> {
	let subscription_entity = subscription_notification.target();
	let notification = subscription_notification.event_mut().clone();

	let mut context = context_param.into_context(subscription_entity);

	context.send_subscription_notification(subscription_entity, notification);
	Ok(())
}

pub(crate) fn erased_subscription_unsubscribe_on_remove<C>(
	deferred_world: DeferredWorld,
	hook_context: HookContext,
) where
	C: Component<Mutability = Mutable> + ErasedEntitySubscriptionHandle,
{
	let context_param: BevySubscriptionContextParam = deferred_world.into();
	let mut context = context_param.into_context(hook_context.entity);

	let target_subscription_entity = context
		.deferred_world
		.get_mut::<C>(hook_context.entity)
		.unwrap()
		.close_and_get_subscription_entity();

	context.send_subscription_notification(
		target_subscription_entity,
		SubscriptionNotification::Unsubscribe,
	);
}

impl UnscheduledEntitySubscriptionHandle {
	pub(crate) fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed_flag: false.into(),
		}
	}
}

impl ErasedEntitySubscriptionHandle for UnscheduledEntitySubscriptionHandle {
	fn close_and_get_subscription_entity(&mut self) -> Entity {
		self.closed_flag.close();
		self.subscription_entity
	}
}

impl UnscheduledSubscriptionHandle for UnscheduledEntitySubscriptionHandle {
	type WeakHandle = WeakEntitySubscriptionHandle;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakEntitySubscriptionHandle::new(self.subscription_entity)
	}
}

impl WithSubscriptionContext for UnscheduledEntitySubscriptionHandle {
	type Context = BevySubscriptionContextProvider;
}

impl Clone for UnscheduledEntitySubscriptionHandle {
	fn clone(&self) -> Self {
		Self {
			subscription_entity: self.subscription_entity,
			closed_flag: self.closed_flag.clone(),
		}
	}
}

impl SubscriptionLike for UnscheduledEntitySubscriptionHandle {
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Unsubscribe,
			);
		}
	}
}

impl TeardownCollection for UnscheduledEntitySubscriptionHandle {
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Add(Some(teardown)),
			);
		} else {
			teardown.execute(context);
		}
	}
}

impl Drop for UnscheduledEntitySubscriptionHandle {
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
