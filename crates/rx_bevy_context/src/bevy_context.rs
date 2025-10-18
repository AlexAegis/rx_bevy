use bevy_ecs::{
	entity::Entity,
	system::{Commands, SystemParam},
	world::DeferredWorld,
};
use rx_core_traits::{
	SignalBound, SubscriberNotification, SubscriptionNotification,
	context::{DropUnsafeSubscriptionContext, SubscriptionContext},
	prelude::SubscriptionContextAccess,
};
use short_type_name::short_type_name;

use crate::{
	ErasedSubscriberEntityAllocator, ScheduledEntitySubscriptionAllocator,
	SubscriberEntityAllocator, SubscriberNotificationEvent, SubscriptionNotificationEvent,
	UnscheduledEntitySubscriptionAllocator,
};

pub struct BevySubscriptionContextProvider;

impl SubscriptionContext for BevySubscriptionContextProvider {
	type Item<'w, 's> = BevySubscriptionContext<'w, 's>;

	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator;

	fn create_context_to_unsubscribe_on_drop<'w, 's>() -> Self::Item<'w, 's> {
		panic!(
			"{}::create_context_to_unsubscribe_on_drop() was called, but its impossible to satisfy!
This is likely due because an active subscription was dropped before it was unsubscribed, which
should automatically happen when its entity despawns!
Please submit an issue at https://github.com/AlexAegis/rx_bevy/issues/new?template=bug_report.md",
			short_type_name::<Self>()
		)
	}
}

#[derive(SystemParam)]
pub struct BevySubscriptionContext<'w, 's> {
	pub commands: Commands<'w, 's>,
	pub deferred_world: DeferredWorld<'w>,
}

impl<'w, 's> BevySubscriptionContext<'w, 's> {
	pub fn send_subscriber_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, BevySubscriptionContextProvider>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let notification_event: SubscriberNotificationEvent<In, InError> = notification.into();
		self.commands.trigger_targets(notification_event, target);
	}

	pub fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification<BevySubscriptionContextProvider>,
	) {
		let notification_event: SubscriptionNotificationEvent = notification.into();
		self.commands.trigger_targets(notification_event, target);
	}
}

impl<'w, 's> SubscriptionContextAccess for BevySubscriptionContext<'w, 's> {
	type SubscriptionContextProvider = BevySubscriptionContextProvider;
}
