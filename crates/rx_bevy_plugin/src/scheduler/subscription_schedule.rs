use std::marker::PhantomData;

use bevy_ecs::{component::Component, schedule::ScheduleLabel};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Component to mark subscriptions with, to trigger `Tick` events without the
/// knowledge of the actual [ObservableComponent]s type
#[derive(Component, Clone, Reflect)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	#[reflect(ignore)]
	_phantom_data: PhantomData<S>,
}
