use core::marker::PhantomData;
use std::any::TypeId;

use bevy_ecs::{component::Component, schedule::ScheduleLabel};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Component to mark subscriptions with, to trigger `Tick` events without the
/// knowledge of the actual [ObservableComponent]s type.
///
/// It is inserted by the `Subscribe` event, users do not need to manually
/// insert this component anywhere.
///
/// It also adds an `ErasedSubscriptionSchedule` that contains TypeId of this
/// component to be used when the schedule has to be cloned without needing to
/// know which schedule was used.
/// TODO: Add C: Clock
#[derive(Component, Clone)]
#[derive_where(Default)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[require(ErasedSubscriptionSchedule::new::<S>())]
pub struct SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	#[cfg_attr(feature = "reflect", reflect(ignore))]
	_phantom_data: PhantomData<S>,
}

/// Contains the TypeId of [SubscriptionSchedule] used on the same entity. It is
/// only insertable by [SubscriptionSchedule] as this is a required component
/// of it and is not creatable otherwise.
///
/// This component is used to clone the schedule component on a subscription
/// without having to know the schedule's type.
#[derive(Component, Clone)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ErasedSubscriptionSchedule {
	subscription_schedule_component_type_id: TypeId,
}

impl ErasedSubscriptionSchedule {
	fn new<S>() -> Self
	where
		S: ScheduleLabel,
	{
		Self {
			subscription_schedule_component_type_id: TypeId::of::<SubscriptionSchedule<S>>(),
		}
	}

	pub fn get_subscription_schedule_component_type_id(&self) -> TypeId {
		self.subscription_schedule_component_type_id
	}
}
