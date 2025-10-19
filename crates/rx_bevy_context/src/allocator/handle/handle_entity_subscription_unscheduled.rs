use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::BevyError,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use rx_core_traits::{
	SubscriptionContext, SubscriptionLike, SubscriptionNotification, Teardown,
	WithSubscriptionContext, allocator::handle::UnscheduledSubscriptionHandle,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ConsumableSubscriptionNotificationEvent, subscription_unsubscribe_on_remove,
};

use super::WeakEntitySubscriptionHandle;

#[derive(Component)]
#[component(on_insert=unscheduled_erased_subscription_add_notification_observer_on_insert, on_remove=subscription_unsubscribe_on_remove)]
pub struct UnscheduledEntitySubscriptionHandle {
	subscription_entity: Entity,
	closed: bool,
}

pub(crate) fn unscheduled_erased_subscription_add_notification_observer_on_insert(
	mut deferred_world: DeferredWorld,
	hook_context: HookContext,
) {
	let mut commands = deferred_world.commands();
	let mut entity_commands = commands.entity(hook_context.entity);
	entity_commands.insert(Observer::new(
		unscheduled_erased_subscription_notification_observer,
	));
}

fn unscheduled_erased_subscription_notification_observer(
	mut subscription_notification: Trigger<ConsumableSubscriptionNotificationEvent>,
	context_param: BevySubscriptionContextParam,
) -> Result<(), BevyError> {
	let subscription_entity = subscription_notification.target();
	let notification = subscription_notification.event_mut().consume();

	let mut context = context_param.into_context(subscription_entity);

	context.send_subscription_notification(subscription_entity, notification);
	Ok(())
}

impl UnscheduledEntitySubscriptionHandle {
	pub(crate) fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed: false,
		}
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
			closed: self.closed,
		}
	}
}

impl SubscriptionLike for UnscheduledEntitySubscriptionHandle {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			self.closed = true;
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Unsubscribe,
			);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Add(teardown),
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
