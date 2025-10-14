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
pub trait WorldStateSubscriptionContextNeo {
	type ActualContext<'world, 'state>: WorldStateContextNeo<'world, 'state>;

	type DropSafety: SubscriptionContextDropSafety;
	type DestinationAllocator<'world, 'state>: DestinationAllocator<
		Context = <Self::ActualContext<'world, 'state> as WorldStateContextNeo<'world, 'state>>::ActualContext,
	>;
	type ErasedDestinationAllocator<'world, 'state>: ErasedDestinationAllocator<
		Context = <Self::ActualContext<'world, 'state> as WorldStateContextNeo<'world, 'state>>::ActualContext,
	>;
	type ScheduledSubscriptionAllocator<'world, 'state>: ScheduledSubscriptionAllocator<
		Context = <Self::ActualContext<'world, 'state> as WorldStateContextNeo<'world, 'state>>::ActualContext,
	>;
	type UnscheduledSubscriptionAllocator<'world, 'state>: UnscheduledSubscriptionAllocator<
		Context = <Self::ActualContext<'world, 'state> as WorldStateContextNeo<'world, 'state>>::ActualContext,
	>;

	fn create_context_to_unsubscribe_on_drop<'world, 'state>() -> Self::ActualContext<'world, 'state>;
}

pub trait WorldStateContextNeo<'world, 'state>: SystemParam {
	type ActualContext: SubscriptionContext;

	/// Creates an entity for this teardown which will execute once it despawns
	fn spawn_teardown_entity(&mut self, teardown: Teardown<Self::ActualContext>) -> Entity;

	fn send_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError, Self::ActualContext>,
	) where
		In: SignalBound,
		InError: SignalBound;
}

pub trait WorldStateContextParamNeo: Sized {
	// type State: Send + Sync + 'static;

	type WorldStateContextNeo<'world, 'state>: WorldStateContextNeo<'world, 'state>;
}

impl<T> WorldStateSubscriptionContextNeo for T
where
	T: WorldStateContextParamNeo,
{
	type DropSafety = DropUnsafeSubscriptionContext;

	type ActualContext<'world, 'state> = T::WorldStateContextNeo<'world, 'state>;

	type DestinationAllocator<'w, 's> = <<Self::ActualContext<'w, 's> as WorldStateContextNeo<
		'w,
		's,
	>>::ActualContext as SubscriptionContext>::DestinationAllocator;
	type ErasedDestinationAllocator<'w, 's> =<<Self::ActualContext<'w, 's> as WorldStateContextNeo<
		'w,
		's,
	>>::ActualContext as SubscriptionContext>::ErasedDestinationAllocator;
	type ScheduledSubscriptionAllocator<'w, 's> =<<Self::ActualContext<'w, 's> as WorldStateContextNeo<
		'w,
		's,
	>>::ActualContext as SubscriptionContext>::ScheduledSubscriptionAllocator;
	type UnscheduledSubscriptionAllocator<'w, 's> =<<Self::ActualContext<'w, 's> as WorldStateContextNeo<
		'w,
		's,
	>>::ActualContext as SubscriptionContext>::UnscheduledSubscriptionAllocator;

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

/* can't bind lifetimes, impossible
impl<'w, 's, T> SubscriptionContext for T
where
	T: WorldStateSubscriptionContextNeo,
{
	type DropSafety = T::DropSafety;

	type DestinationAllocator = T::DestinationAllocator<'w, 's>;
}
*/
