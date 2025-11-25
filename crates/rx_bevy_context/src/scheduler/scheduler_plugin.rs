use core::marker::PhantomData;

use bevy_app::{App, AppExit, Last, Plugin};
use bevy_ecs::{
	entity::Entity,
	query::With,
	resource::Resource,
	schedule::{IntoScheduleConfigs, ScheduleLabel, common_conditions::on_event},
	system::{Commands, Query},
	world::{DeferredWorld, World},
};
use bevy_log::warn;
use bevy_mod_erased_component_registry::AppRegisterErasedComponentExtension;
use bevy_time::Time;
use bevy_window::exit_on_all_closed;
use derive_where::derive_where;
use disqualified::ShortName;
use rx_bevy_common::Clock;
use rx_core_traits::Tick;

use crate::{
	BevySubscriptionContextParam, ScheduledSubscriptionComponent, SubscribeRetryPlugin,
	SubscriptionSchedule, UnfinishedSubscription, execute_pending_retries,
};

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
		// Enables the creation of this component by its TypeId
		app.register_erased_component::<SubscriptionSchedule<S, C>>();

		if !app.is_plugin_added::<SubscribeRetryPlugin>() {
			app.add_plugins(SubscribeRetryPlugin);
		}

		app.add_systems(self.schedule.clone(), tick_all_subscriptions::<S, C>);

		// Just because a subscription is scheduled with `S`, it could be
		// spawned during any other schedule. Therefore the cleanup in the
		// `Last` schedule.
		app.add_systems(Last, clean_unfinished_subscriptions::<S, C>);

		app.add_systems(
			Last,
			unsubscribe_all_subscriptions
				.after(exit_on_all_closed)
				.run_if(on_event::<AppExit>), // TODO(bevy-0.17): on_message
		);
	}
}

fn clean_unfinished_subscriptions<S, C>(
	mut commands: Commands,
	unfinished_subscription_query: Query<
		Entity,
		(
			With<UnfinishedSubscription>,
			With<SubscriptionSchedule<S, C>>,
		),
	>,
) where
	S: ScheduleLabel + Default + Clone,
	C: Clock,
{
	for unfinished_subscription_query in unfinished_subscription_query.iter() {
		warn!(
			"The subscription {} in schedule {} was not populated and does not contain a subscription! It is now being despawned! (This despawn is being issued in the Last schedule!)",
			unfinished_subscription_query,
			ShortName::of::<S>()
		);
		commands.entity(unfinished_subscription_query).despawn();
	}
}

fn unsubscribe_all_subscriptions(world: &mut World) {
	// These could contain stuff that'd panic on drop, better let them execute!
	execute_pending_retries(world);

	let mut subscription_query =
		world.query_filtered::<(Entity, &mut ScheduledSubscriptionComponent), ()>(); // TODO(bevy-0.17): Allow<Internal>
	let mut subscriptions = subscription_query
		.iter_mut(world)
		.map(|(entity, mut subscription_component)| {
			(entity, subscription_component.steal_subscription())
		})
		.collect::<Vec<_>>();

	let mut deferred_world = DeferredWorld::from(world);
	{
		let context_param: BevySubscriptionContextParam = deferred_world.reborrow().into();
		// The entity doesn't really matter during an unsubscription, and it's only there anyway to
		// organize new spawned internal subscriptions
		let mut context = context_param.into_context(None);

		for (_, subscription) in subscriptions.iter_mut() {
			subscription.unsubscribe(&mut context);
		}
	}

	// No need to return stolen subscriptions, the app is closed. We're doing it anyway :)
	for (subscription_entity, subscription) in subscriptions {
		let mut subscription_component = deferred_world
			.get_mut::<ScheduledSubscriptionComponent>(subscription_entity)
			.unwrap();

		subscription_component.return_stolen_subscription(subscription);
	}
}

/// Stores the next tick index for all schedule/clock combinations to avoid
/// conflicts between them.
#[derive(Resource, Default)]
struct RxSchedulerTickIndex {
	index: usize,
}

impl RxSchedulerTickIndex {
	fn get_index_and_increment(&mut self) -> usize {
		let next = self.index;
		self.index += 1;
		next
	}
}

fn tick_all_subscriptions<S, C>(world: &mut World)
where
	S: ScheduleLabel,
	C: Clock,
{
	let index = world
		.get_resource_or_init::<RxSchedulerTickIndex>()
		.get_index_and_increment();
	let time = world.resource::<Time<C>>();
	let tick = Tick {
		index,
		now: time.elapsed(),
		delta: time.delta(),
	};

	let mut subscription_query = world
		.query_filtered::<(Entity, &mut ScheduledSubscriptionComponent), With<SubscriptionSchedule<S, C>>>();
	let mut subscriptions = subscription_query
		.iter_mut(world)
		.map(|(entity, mut subscription_component)| {
			(entity, subscription_component.steal_subscription())
		})
		.collect::<Vec<_>>();

	let mut deferred_world = DeferredWorld::from(world);
	{
		for (entity, subscription) in subscriptions.iter_mut() {
			let context_param: BevySubscriptionContextParam = deferred_world.reborrow().into();
			let mut context = context_param.into_context(Some(*entity));

			subscription.tick(tick.clone(), &mut context);
		}
	}

	for (subscription_entity, subscription) in subscriptions {
		let mut subscription_component = deferred_world
			.get_mut::<ScheduledSubscriptionComponent>(subscription_entity)
			.unwrap();

		subscription_component.return_stolen_subscription(subscription);
	}
}
