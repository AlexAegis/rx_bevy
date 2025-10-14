use std::marker::PhantomData;

use bevy_ecs::{
	entity::Entity,
	system::{Commands, SystemParam},
};
use rx_bevy_core::{
	ObserverNotification, SignalBound, SubscriberNotification, SubscriptionNotification,
	context::{DropUnsafeSubscriptionContext, SubscriptionContext},
	prelude::SubscriptionContextAccess,
};
use short_type_name::short_type_name;

use crate::{
	ErasedSubscriberEntityAllocator, ObserverNotificationEvent,
	ScheduledEntitySubscriptionAllocator, SubscriberEntityAllocator, SubscriberNotificationEvent,
	SubscriptionNotificationEvent, UnscheduledEntitySubscriptionAllocator,
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
	type Item<'c> = BevySubscriptionContext<'c, 'c, ContextAccess>;

	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberEntityAllocator<ContextAccess>;
	type ErasedDestinationAllocator = ErasedSubscriberEntityAllocator<ContextAccess>;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator<ContextAccess>;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator<ContextAccess>;

	fn create_context_to_unsubscribe_on_drop<'c>() -> Self::Item<'c> {
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
	commands: Commands<'w, 's>,
	_phantom_data: PhantomData<fn() -> ContextAccess>,
}

impl<'w, 's, ContextAccess> SubscriptionContextAccess
	for BevySubscriptionContext<'w, 's, ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	type SubscriptionContextProvider = BevySubscriptionContextProvider<ContextAccess>;
}

impl<'world: 'state, 'state: 'world, ContextAccess> EntitySubscriptionContextAccessItem<'world>
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

	fn send_observer_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: ObserverNotification<In, InError>,
	) where
		In: SignalBound,
		InError: SignalBound,
	{
		let notification_event: ObserverNotificationEvent<In, InError> = notification.into();
		self.commands.trigger_targets(notification_event, target);
	}

	fn query_destination(&mut self, target: Entity) {}
}

impl<'world: 'state, 'state: 'world, ContextAccess>
	BevySubscriptionContext<'world, 'state, ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	/*pub fn spawn_teardown_entity(
		&mut self,
		// teardown: Teardown<ContextAccess>,
		teardown: Teardown<BevySubscriptionContextProvider<ContextAccess>>,
	) -> Entity {
		let world_r = move |world: &mut World| {
			let t = teardown;
			let mut state: SystemState<
				<BevySubscriptionContextProvider<ContextAccess> as SubscriptionContext>::Item<'_>,
			> = SystemState::new(world);
			let mut context = state.get_mut(world);
			t.execute(&mut context);
		};

		let teardown_entity = self.commands.spawn_empty();

		let teardown_entity_id = teardown_entity.id();

		teardown_entity_id
	}*/
}
