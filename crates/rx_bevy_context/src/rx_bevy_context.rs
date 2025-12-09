use std::time::Duration;

use bevy_ecs::{
	component::{Component, Mutable},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	world::{DeferredWorld, Mut},
};
use bevy_time::Time;
use disqualified::ShortName;
use rx_bevy_common::Clock;
use rx_core_traits::{
	ContextProvider, ObserverNotification, Signal, SubscriberNotification,
	SubscriptionNotification, TaskContext,
};
use thiserror::Error;

use crate::{RxSignal, SubscriberNotificationEvent, SubscriptionNotificationEvent};

#[derive(Debug)]
pub struct RxBevyContext;

impl ContextProvider for RxBevyContext {
	type Item<'w> = RxBevyContextItem<'w>;
}

/// Use this to acquire the context using the `into_context` fn which extends
/// this system param with additional data. Since a context can be unique for
/// each pushed signal could have it's own "unique" context.
///
/// Currently this is only used for "cosmetic" reasons and isn't actually
/// required for correct operation. But by passing in an Entity too, we can
/// place internally spawned entities relative to another one. The subscriber
/// component on these internally spawned entities are capable of despawning
/// themselves so that's also not a reason to have this. It's purely cosmetic.

pub struct RxBevyContextItem<'w> {
	pub deferred_world: DeferredWorld<'w>,
	now: Duration,
}

impl<'w> TaskContext<'w> for RxBevyContextItem<'w> {
	fn now(&self) -> Duration {
		self.now
	}
}

#[derive(Error, Debug)]
pub enum ContextGetSubscriptionsErasedScheduleError {
	#[error(
		"Attempted to create a ProxySubscription with an incomplete Context! It does not contain a parent subscription entity!"
	)]
	ContextDoesNotHaveASubscritpionEntity,
	#[error("Subscription Entity {0} should have an ErasedSubscriptionSchedule!")]
	SubscriptionEntityDoesNotHaveAnErasedSubscriptionSchedule(Entity),
}

impl<'w> RxBevyContextItem<'w> {
	#[inline]
	pub fn reborrow(&mut self) -> RxBevyContextItem<'_> {
		RxBevyContextItem {
			deferred_world: self.deferred_world.reborrow(),
			now: self.now.clone(),
		}
	}

	pub fn get_expected_component<Comp>(&mut self, destination_entity: Entity) -> &Comp
	where
		Comp: Component,
	{
		let Some(subscriber_component) = self.deferred_world.get::<Comp>(destination_entity) else {
			panic!(
				"{} is missing an expected component: {}!",
				destination_entity,
				ShortName::of::<Comp>(),
			);
		};

		subscriber_component
	}

	pub fn get_expected_component_mut<Comp>(&mut self, destination_entity: Entity) -> Mut<'_, Comp>
	where
		Comp: Component<Mutability = Mutable>,
	{
		let Some(subscriber_component) = self.deferred_world.get_mut::<Comp>(destination_entity)
		else {
			panic!(
				"{} is missing an expected component: {}!",
				destination_entity,
				ShortName::of::<Comp>(),
			);
		};

		subscriber_component
	}

	pub fn try_get_component_mut<Comp>(
		&mut self,
		entity: Entity,
	) -> Result<Mut<'_, Comp>, BevyError>
	where
		Comp: Component<Mutability = Mutable>,
	{
		if let Some(observable_ref) = self.deferred_world.get_mut::<Comp>(entity) {
			Ok(observable_ref)
		} else {
			Err(
				ContextAccessError::NotAnObservable(format!("{}", ShortName::of::<Comp>()), entity)
					.into(),
			)
		}
	}

	pub fn send_observer_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: ObserverNotification<In, InError>,
	) where
		In: Signal,
		InError: Signal,
	{
		let notification_event = RxSignal::<In, InError>::from_notification(notification, target);
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
	}

	pub fn send_subscriber_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError>,
	) where
		In: Signal,
		InError: Signal,
	{
		let notification_event =
			SubscriberNotificationEvent::<In, InError>::from_notification(notification, target);
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
	}

	pub fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification,
	) {
		let notification_event =
			SubscriptionNotificationEvent::from_notification(notification, target);
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
	}
}

#[derive(Error, Debug)]
pub enum ContextAccessError {
	#[error("Tried to get {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
}

pub trait DeferredWorldAsRxBevyContextExtension<'w> {
	fn into_rx_context<'s, C: Clock>(self) -> RxBevyContextItem<'w>;
}

impl<'w> DeferredWorldAsRxBevyContextExtension<'w> for DeferredWorld<'w> {
	fn into_rx_context<'s, C: Clock>(self) -> RxBevyContextItem<'w> {
		let now = self.get_resource::<Time<C>>().unwrap().elapsed();
		RxBevyContextItem {
			deferred_world: self,
			now,
		}
	}
}
