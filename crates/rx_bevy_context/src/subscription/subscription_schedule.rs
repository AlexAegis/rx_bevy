use std::marker::PhantomData;

use bevy_ecs::{component::Component, schedule::ScheduleLabel};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Component to mark subscriptions with, to trigger `Tick` events without the
/// knowledge of the actual [ObservableComponent]s type.
///
/// It is inserted by the `Subscribe` event, users do not need to manually
/// insert this component anywhere.
#[derive(Component, Clone)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<S>,
}
