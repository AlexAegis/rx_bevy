use core::marker::PhantomData;
use std::any::TypeId;

use bevy_ecs::{
	component::{Component, Mutable},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	system::SystemParam,
	world::{DeferredWorld, Mut},
};
use disqualified::ShortName;
use rx_core_traits::{
	DropUnsafeSubscriptionContext, ObserverNotification, SignalBound, Subscriber,
	SubscriberNotification, SubscriptionContext, SubscriptionContextAccess,
	SubscriptionNotification, SubscriptionScheduled, SubscriptionWithTeardown,
	heap_allocator_context::{ErasedSubscriberHeapAllocator, SubscriberHeapAllocator},
};
use stealcell::Stolen;
use thiserror::Error;

use crate::{
	ErasedSubscriptionSchedule, RxSignal, ScheduledEntitySubscriptionAllocator,
	ScheduledSubscriptionComponent, SubscriberComponent, SubscriberNotificationEvent,
	SubscriptionNotificationEvent, UnscheduledEntitySubscriptionAllocator,
	UnscheduledSubscriptionComponent,
};

pub struct BevySubscriptionContextProvider;

impl SubscriptionContext for BevySubscriptionContextProvider {
	type Item<'w, 's> = BevySubscriptionContext<'w, 's>;

	type DropSafety = DropUnsafeSubscriptionContext;

	type DestinationAllocator = SubscriberHeapAllocator<Self>;
	type ErasedDestinationAllocator = ErasedSubscriberHeapAllocator<Self>;
	type ScheduledSubscriptionAllocator = ScheduledEntitySubscriptionAllocator;
	type UnscheduledSubscriptionAllocator = UnscheduledEntitySubscriptionAllocator;

	#[track_caller]
	#[inline]
	fn create_context_to_unsubscribe_on_drop<'w, 's>() -> Self::Item<'w, 's> {
		panic!(
			"{}::create_context_to_unsubscribe_on_drop() was called, but its impossible to satisfy!
This happened because an active subscription was dropped before it was unsubscribed, which
should automatically happen when its entity despawns!
Please submit an issue at https://github.com/AlexAegis/rx_bevy/issues/new?template=bug_report.md
and make sure to include the complete stack trace!",
			ShortName::of::<Self>()
		)
	}
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
#[derive(SystemParam)]
pub struct BevySubscriptionContextParam<'w, 's> {
	pub deferred_world: DeferredWorld<'w>,
	_phantom_data: PhantomData<&'s ()>,
}

impl<'w, 's> BevySubscriptionContextParam<'w, 's> {
	pub fn reborrow(&mut self) -> BevySubscriptionContextParam<'_, '_> {
		BevySubscriptionContextParam {
			deferred_world: self.deferred_world.reborrow(),
			_phantom_data: PhantomData,
		}
	}

	pub fn into_context(self, subscription_entity: Entity) -> BevySubscriptionContext<'w, 's> {
		BevySubscriptionContext {
			deferred_world: self.deferred_world,
			subscription_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<'w, 's> From<DeferredWorld<'w>> for BevySubscriptionContextParam<'w, 's> {
	fn from(deferred_world: DeferredWorld<'w>) -> Self {
		Self {
			deferred_world,
			_phantom_data: PhantomData,
		}
	}
}

pub struct BevySubscriptionContext<'w, 's> {
	pub deferred_world: DeferredWorld<'w>,
	subscription_entity: Entity,
	_phantom_data: PhantomData<&'s ()>,
}

impl<'w, 's> BevySubscriptionContext<'w, 's> {
	pub fn reborrow(&mut self) -> BevySubscriptionContext<'_, '_> {
		BevySubscriptionContext {
			deferred_world: self.deferred_world.reborrow(),
			subscription_entity: self.subscription_entity,
			_phantom_data: PhantomData,
		}
	}

	#[inline]
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}

	pub fn get_subscription_contexts_erased_schedule(&mut self) -> TypeId {
		let erased_subscription_schedule = self
			.deferred_world
			.entity(self.get_subscription_entity())
			.get::<ErasedSubscriptionSchedule>()
			.unwrap_or_else(|| {
				panic!(
					"Subscription Entity {} should have an ErasedSubscriptionSchedule!",
					self.get_subscription_entity()
				)
			});
		erased_subscription_schedule.get_subscription_schedule_component_type_id()
	}

	pub fn get_expected_component<C>(&mut self, destination_entity: Entity) -> &C
	where
		C: Component,
	{
		let Some(subscriber_component) = self.deferred_world.get::<C>(destination_entity) else {
			panic!(
				"{} is missing an expected component: {}!",
				destination_entity,
				ShortName::of::<C>(),
			);
		};

		subscriber_component
	}

	pub fn get_expected_component_mut<C>(&mut self, destination_entity: Entity) -> Mut<'_, C>
	where
		C: Component<Mutability = Mutable>,
	{
		let Some(subscriber_component) = self.deferred_world.get_mut::<C>(destination_entity)
		else {
			panic!(
				"{} is missing an expected component: {}!",
				destination_entity,
				ShortName::of::<C>(),
			);
		};

		subscriber_component
	}

	pub fn try_get_component_mut<C>(&mut self, entity: Entity) -> Result<Mut<'_, C>, BevyError>
	where
		C: Component<Mutability = Mutable>,
	{
		if let Some(observable_ref) = self.deferred_world.get_mut::<C>(entity) {
			Ok(observable_ref)
		} else {
			Err(
				ContextAccessError::NotAnObservable(format!("{}", ShortName::of::<C>()), entity)
					.into(),
			)
		}
	}

	pub fn send_observer_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: ObserverNotification<In, InError>,
	) where
		In: SignalBound,
		InError: SignalBound,
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
		notification: SubscriberNotification<In, InError, BevySubscriptionContextProvider>,
	) where
		In: SignalBound,
		InError: SignalBound,
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
		notification: SubscriptionNotification<BevySubscriptionContextProvider>,
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

	pub fn steal_scheduled_subscription(
		&mut self,
		entity: Entity,
	) -> Result<
		Stolen<
			Box<dyn SubscriptionScheduled<Context = BevySubscriptionContextProvider> + Send + Sync>,
		>,
		BevyError,
	> {
		let mut scheduled_subscription_component =
			self.try_get_component_mut::<ScheduledSubscriptionComponent>(entity)?;

		Ok(scheduled_subscription_component.steal_subscription())
	}

	pub fn return_stolen_scheduled_subscription(
		&mut self,
		entity: Entity,
		subscription: Stolen<
			Box<dyn SubscriptionScheduled<Context = BevySubscriptionContextProvider> + Send + Sync>,
		>,
	) -> Result<(), BevyError> {
		let mut scheduled_subscription_component =
			self.try_get_component_mut::<ScheduledSubscriptionComponent>(entity)?;
		scheduled_subscription_component.return_stolen_subscription(subscription);

		Ok(())
	}

	pub fn steal_unscheduled_subscription<Subscription>(
		&mut self,
		entity: Entity,
	) -> Result<Stolen<Subscription>, BevyError>
	where
		Subscription: 'static
			+ SubscriptionWithTeardown<Context = BevySubscriptionContextProvider>
			+ Send
			+ Sync,
	{
		let mut unscheduled_subscription_component =
			self.try_get_component_mut::<UnscheduledSubscriptionComponent<Subscription>>(entity)?;

		Ok(unscheduled_subscription_component.steal_subscription())
	}

	pub fn return_stolen_unscheduled_subscription<Subscription>(
		&mut self,
		entity: Entity,
		subscription: Stolen<Subscription>,
	) -> Result<(), BevyError>
	where
		Subscription: 'static
			+ SubscriptionWithTeardown<Context = BevySubscriptionContextProvider>
			+ Send
			+ Sync,
	{
		let mut unscheduled_subscription_component =
			self.try_get_component_mut::<UnscheduledSubscriptionComponent<Subscription>>(entity)?;
		unscheduled_subscription_component.return_stolen_subscription(subscription);

		Ok(())
	}

	pub fn steal_subscriber_destination<Destination>(
		&mut self,
		entity: Entity,
	) -> Result<Destination, BevyError>
	where
		Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
		Destination::In: Clone,
		Destination::InError: Clone,
	{
		let mut subscriber_component =
			self.try_get_component_mut::<SubscriberComponent<Destination>>(entity)?;

		Ok(subscriber_component.steal_destination())
	}

	pub fn return_stolen_subscriber_destination<Destination>(
		&mut self,
		entity: Entity,
		destination: Destination,
	) -> Result<(), BevyError>
	where
		Destination: 'static + Subscriber<Context = BevySubscriptionContextProvider> + Send + Sync,
		Destination::In: Clone,
		Destination::InError: Clone,
	{
		let mut subscriber_component =
			self.try_get_component_mut::<SubscriberComponent<Destination>>(entity)?;
		subscriber_component.return_stolen_destination(destination);

		Ok(())
	}
}

impl<'w, 's> SubscriptionContextAccess for BevySubscriptionContext<'w, 's> {
	type SubscriptionContextProvider = BevySubscriptionContextProvider;
}

#[derive(Error, Debug)]
pub enum ContextAccessError {
	#[error("Tried to get {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
}
