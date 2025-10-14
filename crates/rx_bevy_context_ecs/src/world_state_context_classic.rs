use bevy_ecs::{entity::Entity, system::SystemParam};
use rx_bevy_core::{
	SignalBound, SubscriberNotification, Teardown,
	context::{
		DropUnsafeSubscriptionContext, SubscriptionContext,
		allocator::{
			DestinationAllocator, ErasedDestinationAllocator, ScheduledSubscriptionAllocator,
			UnscheduledSubscriptionAllocator,
		},
	},
	prelude::SubscriptionContextDropSafety,
};

use short_type_name::short_type_name;
pub trait WorldStateSubscriptionContext {
	type ActualContext<'world, 'state>: SubscriptionContext<
		DropSafety = DropUnsafeSubscriptionContext,
	>;

	type DropSafety: SubscriptionContextDropSafety;
	type DestinationAllocator<'world, 'state>: DestinationAllocator<
		Context = Self::ActualContext<'world, 'state>,
	>;
	type ErasedDestinationAllocator<'world, 'state>: ErasedDestinationAllocator<
		Context = Self::ActualContext<'world, 'state>,
	>;
	type ScheduledSubscriptionAllocator<'world, 'state>: ScheduledSubscriptionAllocator<
		Context = Self::ActualContext<'world, 'state>,
	>;
	type UnscheduledSubscriptionAllocator<'world, 'state>: UnscheduledSubscriptionAllocator<
		Context = Self::ActualContext<'world, 'state>,
	>;

	fn create_context_to_unsubscribe_on_drop<'world, 'state>() -> Self::ActualContext<'world, 'state>;
}

pub trait WorldStateContext<'world, 'state>:
	SystemParam + SubscriptionContext<DropSafety = DropUnsafeSubscriptionContext>
{
	/// Creates an entity for this teardown which will execute once it despawns
	fn spawn_teardown_entity(&mut self, teardown: Teardown<Self>) -> Entity;

	fn send_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, Self>,
	) where
		In: SignalBound,
		InError: SignalBound;
}

pub trait WorldStateContextParam: Sized {
	// type State: Send + Sync + 'static;

	type WorldStateContext<'world, 'state>: WorldStateContext<'world, 'state>;
}

impl<T> WorldStateSubscriptionContext for T
where
	T: WorldStateContextParam,
{
	type DropSafety = DropUnsafeSubscriptionContext;

	type ActualContext<'world, 'state> = T::WorldStateContext<'world, 'state>;

	type DestinationAllocator<'w, 's> =
		<T::WorldStateContext<'w, 's> as SubscriptionContext>::DestinationAllocator;
	type ErasedDestinationAllocator<'w, 's> =
		<T::WorldStateContext<'w, 's> as SubscriptionContext>::ErasedDestinationAllocator;
	type ScheduledSubscriptionAllocator<'w, 's> =
		<T::WorldStateContext<'w, 's> as SubscriptionContext>::ScheduledSubscriptionAllocator;
	type UnscheduledSubscriptionAllocator<'w, 's> =
		<T::WorldStateContext<'w, 's> as SubscriptionContext>::UnscheduledSubscriptionAllocator;

	fn create_context_to_unsubscribe_on_drop<'world, 'state>() -> Self::ActualContext<'world, 'state>
	{
		panic!(
			"{}::create_context_to_unsubscribe_on_drop() was called, but its impossible to satisfy!
This is likely due because an active subscription was dropped before it was unsubscribed, which
should automatically happen when its entity despawns!
Please submit an issue at https://github.com/AlexAegis/rx_bevy/issues/new?template=bug_report.md",
			short_type_name::<Self>()
		)
	}
}
/*
impl<'w, 's, T> SubscriptionContext for T
where
	T: WorldStateSubscriptionContext,
{
	type DropSafety = T::DropSafety;

	type DestinationAllocator = T::DestinationAllocator<'w, 's>;
}
*/
