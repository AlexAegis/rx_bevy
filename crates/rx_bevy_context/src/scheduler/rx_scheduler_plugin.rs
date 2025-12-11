use core::marker::PhantomData;

use bevy_app::{App, Plugin};
use bevy_ecs::{
	schedule::ScheduleLabel,
	world::{DeferredWorld, Mut, World},
};
use bevy_time::Time;
use derive_where::derive_where;
use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::Tick;

use crate::{DeferredWorldAsRxBevyContextExtension, RxBevyExecutor, SubscribeRetryPlugin};

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
pub struct RxSchedulerPlugin<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	_phantom_data: PhantomData<(S, C)>,
}

impl<S, C> Plugin for RxSchedulerPlugin<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.init_resource::<RxBevyExecutor<S, C>>();

		// TODO: This has to run at the very end of this schedule, or offer a label so users can make sure it's scheduled before the executor
		app.add_systems(S::default(), tick_executor::<S, C>);

		if !app.is_plugin_added::<SubscribeRetryPlugin>() {
			app.add_plugins(SubscribeRetryPlugin);
		}
	}
}

fn tick_executor<S, C>(world: &mut World)
where
	S: ScheduleLabel,
	C: Clock,
{
	let tick = {
		let time = world.resource::<Time<C>>();
		Tick {
			delta: time.delta(),
			elapsed_since_start: time.elapsed(),
		}
	};

	world.resource_scope(|world, mut executor: Mut<RxBevyExecutor<S, C>>| {
		let deferred_world = DeferredWorld::from(world);
		let mut context = deferred_world.into_rx_context::<C>();
		executor.tick(tick, &mut context);
	});
}
