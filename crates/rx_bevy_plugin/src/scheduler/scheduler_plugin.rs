use std::marker::PhantomData;

use bevy_app::{App, AppExit, Last, Plugin};
use bevy_ecs::{
	entity::Entity,
	observer::Observer,
	query::With,
	schedule::{IntoScheduleConfigs, ScheduleLabel, common_conditions::on_event},
	system::{Commands, Query, Res},
	world::{DeferredWorld, World},
};
use bevy_time::Time;
use bevy_window::exit_on_all_closed;
use derive_where::derive_where;
use rx_bevy_common::Clock;
use rx_bevy_context::{
	BevySubscriptionContextParam, ConsumableSubscriptionNotificationEvent,
	ScheduledSubscriptionComponent, SubscriptionNotificationEvent,
};
use rx_core_traits::Tick;

use crate::SubscriptionSchedule;

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
		app.add_systems(
			self.schedule.clone(),
			tick_scheduled_subscriptions_system::<S, C>,
		);

		app.add_systems(
			Last,
			trigger_unsubscribe_all_subscriptions::<S>
				.after(exit_on_all_closed)
				.run_if(on_event::<AppExit>),
		);
	}
}

/// Sends a tick notification for all subscriptions scheduled with this schedule
pub fn trigger_unsubscribe_all_subscriptions<S: ScheduleLabel>(
	mut commands: Commands,
	subscription_query: Query<Entity, With<SubscriptionSchedule<S>>>,
) {
	let subscriptions = subscription_query.iter().collect::<Vec<_>>();

	if !subscriptions.is_empty() {
		let consumable_notification: ConsumableSubscriptionNotificationEvent =
			SubscriptionNotificationEvent::Unsubscribe.into();
		commands.trigger_targets(consumable_notification, subscriptions);
	}
}

fn unsubscribe_all_subscriptions(world: &mut World) {
	let mut subscription_query = world.query::<(Entity, &mut ScheduledSubscriptionComponent)>();
	let mut subscriptions = subscription_query
		.iter_mut(world)
		.map(|(entity, mut subscription_context)| {
			(entity, subscription_context.steal_subscription())
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
	subscription_query: Query<Entity, (With<SubscriptionSchedule<S>>, With<Observer>)>,
) {
	let subscriptions = subscription_query.iter().collect::<Vec<_>>();

	if !subscriptions.is_empty() {
		let consumable_notification: ConsumableSubscriptionNotificationEvent =
			SubscriptionNotificationEvent::Tick(Tick {
				now: time.elapsed(),
				delta: time.delta(),
			})
			.into();

		commands.trigger_targets(consumable_notification, subscriptions);
	}
}
