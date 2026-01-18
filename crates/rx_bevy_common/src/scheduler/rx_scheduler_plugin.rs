use bevy_app::{App, Plugin};
use bevy_ecs::{
	schedule::ScheduleLabel,
	world::{DeferredWorld, Mut, World},
};
use bevy_time::Time;
use derive_where::derive_where;
use rx_core_common::PhantomInvariant;
use rx_core_scheduler_ticking::Tick;

use crate::{Clock, RxBevyExecutor, SubscribeRetryPlugin};

/// # [RxSchedulerPlugin]
///
/// Executes scheduled work issued by subscriptions.
/// You need to add this plugin for every `S` bevy schedule (`Update`,
/// `PostUpdate`) and `C` clock (`Virtual`, `Real`) combination you're using
/// for [`RxSchedule`][crate::RxSchedule]s within your app.
///
/// Don't forget to add the base [`RxPlugin`][crate::RxPlugin] too!
///
/// The executor system that ticks scheduler work is simply added to the
/// generic `S` bevy schedule!
///
/// In case you have other systems relying on scheduled work already being
/// executed, consider creating a custom bevy schedule for the rx scheduler.
/// Only do this if you know you need it! It's perfectly fine to just use
/// the `Update` schedule for the rx scheduler, you still have `PreUpdate`
/// and `PostUpdate` as schedules where you know you're before/after the
/// subscriptions being ticked.
///
/// ## When do signals happen?
///
/// When exactly an observable emits depends on its implementation:
/// - Immediately when the subscription happens
///   
///   > Observables like `just` emits instantly, which means if you subscribe
///   > to one directly, it happens exactly when you call subscribe on it.
///   > If you subscribe to it using a command, then it happens when the
///   > command is executed.
///   > ("directly" here means using the observables `subscribe` method, and
///   > not through `Commands`).
/// - Or if it uses a scheduler (you know which one uses a scheduler because
///   you have to give it a `SchedulerHandle`), then it can also emit
///   signals whenever that scheduler gets "ticked". Which happens
///   here in the `rx_executor` system defined by this plugin.
///
///   > For example, let's say you subscribe directly to an `interval`
///   > observable from a system running once under the `PreUpdate` schedule,
///   > and the interval has the option `start_on_subscribe: true`. Then, the
///   > first emission happens immediately in that system in `PreUpdate`, but
///   > the following emissions, since they are all sheduled, will happen in
///   > the schedule of the handler you gave it. Which could be anything!
#[derive_where(Default)]
pub struct RxSchedulerPlugin<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	_phantom_data: PhantomInvariant<(S, C)>,
}

impl<S, C> Plugin for RxSchedulerPlugin<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.init_resource::<RxBevyExecutor<S, C>>();

		app.add_systems(S::default(), rx_executor::<S, C>);

		if !app.is_plugin_added::<SubscribeRetryPlugin>() {
			app.add_plugins(SubscribeRetryPlugin);
		}
	}
}

/// This system executes scheduled work for the `S` bevy schedule using the `C`
/// clock.
///
/// You may refer to this system for ordering purposes but also consider using
/// a custom schedule too.
pub fn rx_executor<S, C>(world: &mut World)
where
	S: ScheduleLabel,
	C: Clock,
{
	let tick = Tick::new(world.resource::<Time<C>>().elapsed());

	world.resource_scope(|world, mut executor: Mut<RxBevyExecutor<S, C>>| {
		let deferred_world = DeferredWorld::from(world);
		let mut context = deferred_world.into();
		executor.tick_to(tick, &mut context);
	});
}
