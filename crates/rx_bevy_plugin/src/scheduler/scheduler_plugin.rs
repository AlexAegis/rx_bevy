use std::marker::PhantomData;

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::{
	entity::Entity,
	observer::Observer,
	query::With,
	schedule::ScheduleLabel,
	system::{Commands, Query, Res, ResMut},
};
use bevy_log::trace;
use bevy_reflect::{GetTypeRegistration, Reflect, Reflectable};
use bevy_time::Time;
use derive_where::derive_where;
use rx_bevy_common_bounds::Clock;
use rx_bevy_observable::Tick;

use crate::{SubscriptionSchedule, SubscriptionScheduleRegistry};

/// An RxScheduler is responsible to keep active, scheduled Subscriptions emitting
/// values.
///
/// > For example, an interval observable needs to re-emit events again and again
/// > in set intervals, and the scheduler is responsible for "ticking" these,
/// > and at each tick it can do something, it will do something.
///
/// > On the contrary, a simple, non-scheduled observable - like one that provides
/// > keyboard presses as observable events - does not need any scheduling. These
/// > events propagate through subscriptions as they happen.
///
/// An RxScheduler is tied to a regular bevy Schedule, and all it does is call
/// `tick` on [SubscriptionComponent]s at the schedule they are implemented for.
#[derive_where(Default)]
pub struct RxScheduler<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	schedule: S,
	_phantom_data: PhantomData<C>,
}

impl<S, C> Plugin for RxScheduler<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.world_mut()
			.register_component::<SubscriptionSchedule<S>>();

		app.init_resource::<SubscriptionScheduleRegistry>();

		trace!(
			"Build RxScheduler for Schedule: {}, Clock: {}",
			short_type_name::short_type_name::<S>(),
			short_type_name::short_type_name::<C>()
		);
		app.add_systems(Startup, register_schedule::<S>);
		app.add_systems(self.schedule.clone(), tick_subscriptions_system::<S, C>);
	}
}

fn register_schedule<S>(mut registry: ResMut<SubscriptionScheduleRegistry>)
where
	S: ScheduleLabel + Default,
{
	println!("Register Schedule");
	registry.register::<SubscriptionSchedule<S>>();
}

pub fn tick_subscriptions_system<S: ScheduleLabel, C: Clock>(
	mut commands: Commands,
	time: Res<Time<C>>,
	subscription_query: Query<
		Entity,
		(
			With<SubscriptionSchedule<S>>,
			With<Observer>, // The tick Observer, which is optional for non tickable Subscribers
		),
	>,
) {
	let subscriptions = subscription_query.iter().collect::<Vec<_>>();

	commands.trigger_targets(Tick::new(&time), subscriptions);
}
