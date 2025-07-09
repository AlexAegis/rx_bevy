use std::marker::PhantomData;

use bevy_ecs::{
	component::Component,
	entity::Entity,
	event::Event,
	name::Name,
	observer::{Observer, Trigger},
	schedule::ScheduleLabel,
	system::{Commands, Query},
};
use bevy_log::{error, trace, warn};

use short_type_name::short_type_name;
use thiserror::Error;

use crate::{
	ObservableComponent, ObservableSignalBound, RelativeEntity, RxTick, ScheduledSubscription,
	SubscriptionComponent, SubscriptionEntityContext, SubscriptionSchedule, Subscriptions,
};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Event)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Subscribe<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	subscriber_entity: RelativeEntity,
	/// This entity can only be spawned from this events constructors
	subscription_entity: Entity,
	scheduled: bool,
	_phantom_data: PhantomData<O>,
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarker;

pub fn on_subscribe<O>(
	trigger: Trigger<Subscribe<O>>,
	mut observable_component_query: Query<(&mut O, Option<&mut Subscriptions<O>>)>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = trigger.target();
	println!("on_observable_subscribe {}", observable_entity);
	let Ok((mut observable_component, existing_subscriptions_component)) =
		observable_component_query.get_mut(observable_entity)
	else {
		warn!(
			"Tried to subscribe to {} but it does not exist on {}",
			short_type_name::<O>(),
			observable_entity
		);
		return; // Err(SubscribeError::NotAnObservable.into());
	};
	let destination_entity = trigger.get_subscriber_entity_or_this(observable_entity);

	// Observables that re-emit everything they observe should not be able to
	// subscribe to themselves as that would cause an infinite loop
	if !O::CAN_SELF_SUBSCRIBE && observable_entity == destination_entity {
		warn!(
			"Tried to subscribe to itself when it is disallowed! {}({})",
			short_type_name::<O>(),
			observable_entity
		);
		return; // Err(SubscribeError::SelfSubscribeDisallowed.into());
	}

	if O::Subscription::SCHEDULED && !trigger.event().is_scheduled() {
		error!(
			"Tried to subscribe to a scheduled observable with an unscheduled Subscription! {}({})",
			short_type_name::<O>(),
			observable_entity
		);
		return; // Err(SubscribeError::UnscheduledSubscribeOnScheduledObservable.into());
	}

	// Get the pre-spawned scheduled Subscription entity
	let subscription_entity = trigger.event().get_subscription_entity();

	// Initialize the Subscriptions component on the observable
	if let Some(mut subscriptions) = existing_subscriptions_component {
		// Technically a required component, but [ObservableComponent] is a trait, so it's inserted lazily
		subscriptions.push(subscription_entity);
	} else {
		commands
			.entity(observable_entity)
			.insert(Subscriptions::<O>::new(subscription_entity));
	}

	{
		let context = SubscriptionEntityContext {
			observable_entity,
			subscriber_entity: destination_entity,
			subscription_entity,
		};

		let scheduled_subscription =
			observable_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscription_entity_commands = commands.entity(subscription_entity);
		subscription_entity_commands.insert((
			Name::new(format!(
				"Observer (Subscription) {}({})",
				short_type_name::<O>(),
				observable_entity
			)),
			SubscriptionComponent::<O>::new(
				observable_entity,
				destination_entity,
				scheduled_subscription,
			),
		));

		if O::Subscription::SCHEDULED {
			subscription_entity_commands.insert((
				SubscriptionMarker,
				Observer::new(subscription_tick_observer::<O>).with_entity(subscription_entity), // It's observing itself!
			));
		};
	}
}

/// This is what would drive an "intervalObserver" ticking a subscriber,
/// that will decide if it should next something to its subscribers or not
///
/// Notice how the schedule is not present. The [RxScheduler] plugin will
/// query based on the Schedule but the Subscription itself does not have to be
/// aware of the Schedule it runs on.
pub fn subscription_tick_observer<O>(
	trigger: Trigger<RxTick>,
	mut subscription_query: Query<&mut SubscriptionComponent<O>>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound + Clone,
	O::OutError: ObservableSignalBound,
{
	#[cfg(feature = "debug")]
	trace!("subscription_tick_observer {:?}", trigger.event());

	if let Ok(mut subscription) = subscription_query.get_mut(trigger.target()) {
		let context = subscription
			.get_subscription_entity_context(trigger.target())
			.upgrade(&mut commands);
		subscription.tick(trigger.event(), context);
	}
}

impl<O> Subscribe<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	pub fn get_subscriber_entity_or_this(&self, or_another: Entity) -> Entity {
		self.subscriber_entity.this_or(or_another)
	}

	/// Be aware that if you can't subscribe to a scheduled observable
	/// with an unscheduled subscribe request
	pub fn unscheduled(
		subscriber_entity: RelativeEntity,
		commands: &mut Commands,
	) -> (Self, Entity) {
		let subscription_entity = commands.spawn_empty().id();

		(
			Self {
				subscriber_entity,
				subscription_entity,
				scheduled: false,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn scheduled<S>(
		subscriber_entity: RelativeEntity,
		commands: &mut Commands,
	) -> (Self, Entity)
	where
		S: ScheduleLabel,
	{
		let subscription_entity = commands.spawn(SubscriptionSchedule::<S>::default()).id();

		(
			Self {
				subscriber_entity,
				subscription_entity,
				scheduled: true,
				_phantom_data: PhantomData,
			},
			subscription_entity,
		)
	}

	pub fn is_scheduled(&self) -> bool {
		self.scheduled
	}

	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
	}
}

/// TODO: Currently unused, could be used once bevy observers become fallible
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to an entity that does not contain an ObservableComponent")]
	NotAnObservable,
	#[error(
		"Tried to subscribe to an ObservableComponent which disallows subscriptions from the same entity"
	)]
	SelfSubscribeDisallowed,
	#[error("Tried to subscribe to a scheduled observable with an unscheduled Subscription!")]
	UnscheduledSubscribeOnScheduledObservable,
}
