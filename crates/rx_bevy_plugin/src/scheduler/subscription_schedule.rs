use std::marker::PhantomData;

use bevy_ecs::{component::Component, schedule::ScheduleLabel};
use bevy_reflect::Reflect;
use derive_where::derive_where;

/// Component to mark subscriptions with, to trigger `Tick` events without the
/// knowledge of the actual [ObservableComponent]s type
#[derive(Component)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	_phantom_data: PhantomData<S>,
}
