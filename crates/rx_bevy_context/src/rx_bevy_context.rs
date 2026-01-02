use bevy_ecs::{
	component::{Component, Mutable},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	world::{DeferredWorld, Mut},
};
use disqualified::ShortName;
use rx_core_traits::{
	ObserverNotification, Signal, SubscriberNotification, SubscriptionNotification, WorkContext,
	WorkContextProvider,
};
use thiserror::Error;

use crate::{RxSignal, SubscriberNotificationEvent, SubscriptionNotificationEvent};

#[derive(Debug)]
pub struct RxBevyContext;

impl WorkContextProvider for RxBevyContext {
	type Item<'w> = RxBevyContextItem<'w>;
}

pub struct RxBevyContextItem<'w> {
	pub deferred_world: DeferredWorld<'w>,
}

impl<'w> WorkContext<'w> for RxBevyContextItem<'w> {}

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

impl<'w> From<DeferredWorld<'w>> for RxBevyContextItem<'w> {
	fn from(deferred_world: DeferredWorld<'w>) -> Self {
		RxBevyContextItem { deferred_world }
	}
}
