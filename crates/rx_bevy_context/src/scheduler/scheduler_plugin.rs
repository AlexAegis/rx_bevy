use core::marker::PhantomData;

use bevy_app::{App, AppExit, Last, Plugin};
use bevy_ecs::{
	entity::Entity,
	entity_disabling::Internal,
	observer::Observer,
	query::{Allow, With},
	schedule::{IntoScheduleConfigs, ScheduleLabel, common_conditions::on_message},
	system::{Commands, Local, Query, Res},
	world::{DeferredWorld, World},
};
use bevy_mod_erased_component_registry::AppRegisterErasedComponentExtension;
use bevy_time::Time;
use bevy_window::exit_on_all_closed;
use derive_where::derive_where;
use rx_bevy_common::Clock;
use rx_core_traits::{SubscriptionNotification, Tick};

use crate::{
	BevySubscriptionContextParam, ScheduledSubscriptionComponent, SubscriptionNotificationEvent,
	SubscriptionSchedule,
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
		app.register_erased_component::<SubscriptionSchedule<S>>();

		app.add_systems(
			self.schedule.clone(),
			tick_scheduled_subscriptions_system::<S, C>,
		);

		app.add_systems(
			Last,
			unsubscribe_all_subscriptions
				.after(exit_on_all_closed)
				.run_if(on_message::<AppExit>),
		);
	}
}

fn unsubscribe_all_subscriptions(world: &mut World) {
	let mut subscription_query =
		world.query_filtered::<(Entity, &mut ScheduledSubscriptionComponent), Allow<Internal>>();
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
		let mut context = context_param.into_context(Entity::PLACEHOLDER);

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

/// Sends a tick notification for all subscriptions scheduled with this schedule
pub fn tick_scheduled_subscriptions_system<S: ScheduleLabel, C: Clock>(
	mut commands: Commands,
	time: Res<Time<C>>,
	subscription_query: Query<
		Entity,
		(
			With<SubscriptionSchedule<S>>,
			With<Observer>,
			Allow<Internal>,
		),
	>,
	mut index: Local<usize>,
) {
	let subscription_entities = subscription_query.iter().collect::<Vec<_>>();

	if !subscription_entities.is_empty() {
		let tick = Tick {
			index: *index,
			now: time.elapsed(),
			delta: time.delta(),
		};
		*index += 1;

		for event in subscription_entities.iter().map(|target| {
			SubscriptionNotificationEvent::from_notification(
				SubscriptionNotification::Tick(tick.clone()),
				*target,
			)
		}) {
			commands.trigger(event);
		}
	}
}

//TODO: Evaluate if not using commands would be better or not, maybe from a scheduling perspective for users? could say .after this to know when its settled
/*
fn tick_all_subscriptions<S, C>(world: &mut World)
where
	S: ScheduleLabel,
	C: Clock,
{
	let time = world.resource::<Time<C>>();
	let tick = Tick {
		index: 0, // TODO: Wrong
		now: time.elapsed(),
		delta: time.delta(),
	};

	let mut subscription_query = world.query::<(Entity, &mut ScheduledSubscriptionComponent)>();
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
			let mut context = context_param.into_context(*entity);

			subscription.tick(tick.clone(), &mut context);
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
*/
