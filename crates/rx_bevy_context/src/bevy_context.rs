use std::marker::PhantomData;

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
	context::{EntitySubscriptionContextAccessItem, EntitySubscriptionContextAccessProvider},
};

pub struct BevySubscriptionContextProvider<ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn() -> ContextAccess>,
}

impl<ContextAccess> SubscriptionContext for BevySubscriptionContextProvider<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Item<'w, 's> = BevySubscriptionContext<'w, 's, ContextAccess>;

	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator<ContextAccess>;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator<ContextAccess>;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator<ContextAccess>;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator<ContextAccess>;

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
pub struct BevySubscriptionContext<'w, 's, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	pub commands: Commands<'w, 's>,
	pub deferred_world: DeferredWorld<'w>,
	// TODO: SystemParam doesn't like this either, time to simplify. And it's not like they could be merged, so it's useless
	//asd: StaticSystemParam<
	//	'w,
	//	's,
	//	<ContextAccess as EntitySubscriptionContextAccessProvider>::Item<'w, 's>,
	//>,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<'w, 's, ContextAccess> BevySubscriptionContext<'w, 's, ContextAccess> where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider
{
}

impl<'w, 's, ContextAccess> SubscriptionContextAccess
	for BevySubscriptionContext<'w, 's, ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	type SubscriptionContextProvider = BevySubscriptionContextProvider<ContextAccess>;
}

impl<'world, 'state, ContextAccess> EntitySubscriptionContextAccessItem<'world, 'state>
	for BevySubscriptionContext<'world, 'state, ContextAccess>
where
	ContextAccess: 'state + EntitySubscriptionContextAccessProvider,
{
	type AccessProvider = ContextAccess;

	fn send_subscriber_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<
			In,
			InError,
			BevySubscriptionContextProvider<Self::AccessProvider>,
		>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let notification_event: SubscriberNotificationEvent<In, InError, ContextAccess> =
			notification.into();
		self.commands.trigger_targets(notification_event, target);
	}

	fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification<
			BevySubscriptionContextProvider<Self::AccessProvider>,
		>,
	) {
		let notification_event: SubscriptionNotificationEvent<ContextAccess> = notification.into();
		self.commands.trigger_targets(notification_event, target);
	}
}
