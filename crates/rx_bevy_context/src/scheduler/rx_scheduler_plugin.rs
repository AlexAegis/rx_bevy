use core::marker::PhantomData;

use bevy_app::{App, AppExit, Last, Plugin};
use bevy_ecs::{
	entity::Entity,
	query::With,
	schedule::{IntoScheduleConfigs, ScheduleLabel, common_conditions::on_event},
	system::{Commands, Query},
	world::{DeferredWorld, Mut, World},
};
use bevy_log::warn;
use bevy_time::{Time, Virtual};
use bevy_window::exit_on_all_closed;
use derive_where::derive_where;
use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::Tick;
use rx_core_traits::SubscriptionLike;

use crate::{
	DeferredWorldAsRxBevyContextExtension, RxBevyExecutor, SubscribeRetryPlugin,
	SubscriptionComponent, UnfinishedSubscription, execute_pending_retries,
};

/// Used for cleanup tasks like despawning entities
pub type RxBevyExecutorLast = RxBevyExecutor<Last, Virtual>;
pub type RxBevySubscriptionScheduler = RxScheduler<Last, Virtual>;

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
	_phantom_data: PhantomData<(S, C)>,
}

impl<S, C> Plugin for RxScheduler<S, C>
where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	fn build(&self, app: &mut App) {
		app.init_resource::<RxBevyExecutor<S, C>>();

		if !app.is_plugin_added::<RxBevySubscriptionScheduler>() {
			app.add_plugins(RxBevySubscriptionScheduler::default());
		}

		// TODO: Maybe as part of a base plugin?
		app.add_systems(Last, clean_unfinished_subscriptions);

		// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
		// TODO: This has to run at the very end of this schedule, or offer a label so users can make sure it's scheduled before the executor
		app.add_systems(S::default(), tick_executor::<S, C>);

		if !app.is_plugin_added::<SubscribeRetryPlugin>() {
			app.add_plugins(SubscribeRetryPlugin);
		}

		app.add_systems(
			Last,
			unsubscribe_all_subscriptions
				.after(exit_on_all_closed)
				.run_if(on_event::<AppExit>), // TODO(bevy-0.17): on_message
		);
	}
}

fn clean_unfinished_subscriptions(
	mut commands: Commands,
	unfinished_subscription_query: Query<Entity, With<UnfinishedSubscription>>,
) {
	for unfinished_subscription_query in unfinished_subscription_query.iter() {
		warn!(
			"The subscription {} was not populated and does not contain a subscription! It is now being despawned! (This despawn is being issued in the Last schedule!)",
			unfinished_subscription_query
		);
		commands.entity(unfinished_subscription_query).despawn();
	}
}

fn unsubscribe_all_subscriptions(world: &mut World) {
	// These could contain stuff that'd panic on drop, better let them execute!
	execute_pending_retries(world);

	let mut subscription_query = world.query::<&mut SubscriptionComponent>(); // TODO(bevy-0.17): Allow<Internal>

	for mut subscription in subscription_query.iter_mut(world) {
		subscription.unsubscribe();
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
