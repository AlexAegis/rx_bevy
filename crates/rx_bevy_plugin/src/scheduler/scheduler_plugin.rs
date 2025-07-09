use bevy::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;

use crate::{RxTick, SubscriptionSchedule};

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
///
/// TODO: Do Clocks tie into schedulers or just subscriptions?
pub struct RxScheduler<S>
where
	S: ScheduleLabel + Clone,
{
	schedule: S,
}

impl<S> RxScheduler<S>
where
	S: ScheduleLabel + Clone,
{
	pub fn on(schedule: S) -> Self {
		Self { schedule }
	}
}

impl<S> Plugin for RxScheduler<S>
where
	S: ScheduleLabel + Clone,
{
	fn build(&self, app: &mut App) {
		app.add_systems(self.schedule.clone(), tick_subscriptions_system::<S>);
	}
}

// TODO: Add clocks
pub fn tick_subscriptions_system<S: ScheduleLabel>(
	mut commands: Commands,
	time: Res<Time>,
	subscription_query: Query<
		Entity,
		(
			With<SubscriptionSchedule<S>>,
			With<bevy::ecs::prelude::Observer>, // The tick Observer, which is optional for non tickable Subscribers
		),
	>,
) {
	let subscriptions = subscription_query.iter().collect::<Vec<_>>();

	commands.trigger_targets(RxTick::new(&time), subscriptions);
}
