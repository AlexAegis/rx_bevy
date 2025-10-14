use bevy_ecs::{
	entity::Entity,
	system::{Commands, SystemParam},
};
use rx_bevy_core::{
	SignalBound, SubscriberNotification, Teardown,
	prelude::{DropUnsafeSubscriptionContext, SubscriptionContext},
};

use crate::{
	EntitySubscription, ErasedSubscriberEntityAllocator, IntoCommandSubscriberNotification,
	ScheduledEntitySubscriptionAllocator, SubscriberEntityAllocator,
	UnscheduledEntitySubscriptionAllocator,
};

#[derive(SystemParam)]
pub struct BevySubscriberContext<'w, 's> {
	commands: Commands<'w, 's>,
}

impl<'world, 'state> BevySubscriberContext<'world, 'state> {
	pub fn spawn_teardown_entity(&mut self, mut teardown: Teardown<Self>) -> Entity {
		let mut teardown_entity = self.commands.spawn_empty();
		let teardown_component =
			EntitySubscription::new_with_teardown(teardown_entity.id(), teardown);
		let teardown_entity_id = teardown_entity.id();
		//if let Some(asd) = teardown.take() {
		//	let c = TeardownCommand::<Self>::new(asd);
		//	self.commands.queue(c);
		//}

		teardown_entity.insert(teardown_component);
		teardown_entity_id
	}

	pub fn send_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, Self>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let mapped_notification = notification.into_command_subscriber_notification(self);
		self.commands.trigger_targets(mapped_notification, target);
	}
}

impl<'w, 's> SubscriptionContext for BevySubscriberContext<'w, 's> {
	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator<'w, 's>;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator<'w, 's>;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator<'w, 's>;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator<'w, 's>;

	fn create_context_to_unsubscribe_on_drop() -> Self {
		panic!()
	}
}
