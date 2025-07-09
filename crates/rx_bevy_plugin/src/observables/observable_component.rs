use bevy::{ecs::component::Mutable, prelude::*};
use bevy_ecs::{component::HookContext, schedule::ScheduleLabel, world::DeferredWorld};
use derive_where::derive_where;
use rx_bevy::ObservableOutput;
use short_type_name::short_type_name;
use std::{fmt::Debug, marker::PhantomData};
use thiserror::Error;

use crate::{
	DebugBound, ObservableSignalBound, RxTick, ScheduledSubscription, SubscriptionComponent,
	Subscriptions,
};

/// TODO: While this is required for all ObservableComponents, it's a separate trait
/// to be the auto-implemented by a macro.
///
/// This is technically a one-on-one relationship, each ObservableComponent has
/// exactly one other entity listening for [Subscribe] events
pub trait WithSubscribeObserverReference {
	/// Should return the entity reference to the entity that observes [Subscribe]
	/// events for this observable
	fn get_subscribe_observer_entity(&self) -> Option<Entity>;

	/// Returns the previous observer entity, if exists.
	/// (Implement as `.replace` on the stored `Option<Entity>`)
	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity>;
}

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
pub trait ObservableComponent:
	ObservableOutput + Component<Mutability = Mutable> + WithSubscribeObserverReference + DebugBound
where
	Self::Out: Send + Sync + DebugBound,
	Self::OutError: Send + Sync + DebugBound,
{
	const CAN_SELF_SUBSCRIBE: bool;

	/// If the Observable does not need any scheduling, use [NonScheduledSubscription]
	/// Otherwise implement a [ScheduledSubscription] that can emit events when
	/// ticked by an [RxScheduler].
	type Subscription: ScheduledSubscription<Out = Self::Out, OutError = Self::OutError>
		+ Send
		+ Sync;

	fn on_insert(&mut self, context: ObservableOnInsertContext);

	fn on_subscribe(&mut self, context: SubscriptionContext) -> Self::Subscription;
}

#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct NonScheduledSubscription<Out, OutError>
where
	Out: 'static + Send + Sync + DebugBound,
	OutError: 'static + Send + Sync + DebugBound,
{
	_phantom_data: PhantomData<(Out, OutError)>,
}

impl<Out, OutError> ObservableOutput for NonScheduledSubscription<Out, OutError>
where
	Out: 'static + Send + Sync + DebugBound,
	OutError: 'static + Send + Sync + DebugBound,
{
	type Out = Out;
	type OutError = OutError;
}

impl<Out, OutError> ScheduledSubscription for NonScheduledSubscription<Out, OutError>
where
	Out: 'static + Send + Sync + DebugBound,
	OutError: 'static + Send + Sync + DebugBound,
{
	const SCHEDULED: bool = false;

	fn on_tick(&mut self, _event: &RxTick, _context: SubscriptionContext) {
		unreachable!()
	}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {}
}

#[derive_where(Debug)]
pub struct ObservableOnInsertContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
}

#[derive_where(Debug)]
pub struct SubscriptionContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

#[derive(Debug)]
pub struct SubscriptionEntityContext {
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

impl SubscriptionEntityContext {
	pub fn upgrade<'a, 'w, 's>(
		self,
		commands: &'a mut Commands<'w, 's>,
	) -> SubscriptionContext<'a, 'w, 's> {
		SubscriptionContext {
			commands,
			observable_entity: self.observable_entity,
			subscriber_entity: self.subscriber_entity,
			subscription_entity: self.subscription_entity,
		}
	}
}

// TODO: So that you can just .next stuff instead of emitting values by hand
//impl Observer for SubscriptionContext {}

/// TODO: Add on remove hooks to despawn this and the observable component together, the observable should be removed when this is removed, and when the observable is removed this entire entity should despawn
#[derive(Component, Reflect)]
pub struct SubscribeObserverComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	observable_entity: Entity,
	_phantom_data: PhantomData<O>,
}

impl<O> SubscribeObserverComponent<O>
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	pub fn new(observable_entity: Entity) -> Self {
		Self {
			observable_entity,
			_phantom_data: PhantomData,
		}
	}
}

/// This on_insert hook sets up the observable so it can spawn new subscriptions
/// upon receiving [Subscribe] events.
pub fn on_observable_insert_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = hook_context.entity;

	// This is the observer that processes [Subscribe] events for this specific observable.
	// It will be despawned when the observable is removed.
	let subscribe_observer_entity = {
		let mut commands = deferred_world.commands();
		debug!(
			"setting up subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);

		commands
			.spawn((
				ChildOf(observable_entity), // Purely for organizational purposes in debug views like WorldInspector
				SubscribeObserverComponent::<O>::new(observable_entity),
				Name::new(format!(
					"Observer (Observable Subscribe) - {}({}) ",
					short_type_name::<O>(),
					observable_entity
				)),
				bevy_ecs::prelude::Observer::new(on_observable_subscribe::<O>)
					.with_entity(observable_entity),
			))
			.id()
	};

	{
		let (mut entities, mut commands) = deferred_world.entities_and_commands();
		let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();

		let mut component = observable_entity_mut.get_mut::<O>().unwrap();
		component.set_subscribe_observer_entity(subscribe_observer_entity);

		component.on_insert(ObservableOnInsertContext {
			observable_entity,
			commands: &mut commands,
		});
	}
}

/// To achieve a one-on-one relationship, the observer that observes [Subscribe] events
/// is despawned when the observable component is removed
pub fn on_observable_remove_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = hook_context.entity;
	let (mut entities, mut commands) = deferred_world.entities_and_commands();
	let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();
	let observable_component = observable_entity_mut.get_mut::<O>().unwrap();

	if let Some(subscribe_observer_entity) = observable_component.get_subscribe_observer_entity() {
		debug!(
			"despawning subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);
		commands.entity(subscribe_observer_entity).despawn();
	}
}

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

pub fn on_observable_subscribe<O>(
	trigger: Trigger<SubscribeFor<O>>,
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
	let destination_entity = trigger.subscriber_entity.resolve(observable_entity);

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
		let scheduled_subscription = observable_component.on_subscribe(SubscriptionContext {
			commands: &mut commands,
			observable_entity,
			subscriber_entity: destination_entity,
			subscription_entity,
		});

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
				bevy::ecs::prelude::Observer::new(subscription_tick_observer::<O>)
					.with_entity(subscription_entity), // It's observing itself!
			));
		};
	}
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarker;

/// Erased type to trigger `Tick` events without the knowledge of the actual Observables type
#[derive(Component)]
#[derive_where(Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionSchedule<S>
where
	S: ScheduleLabel,
{
	_phantom_data: PhantomData<S>,
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

#[derive(Debug)]
pub enum SubscriberEntity {
	This,
	Other(Entity),
}

impl SubscriberEntity {
	pub fn resolve(&self, observable_entity: Entity) -> Entity {
		match self {
			Self::Other(entity) => *entity,
			Self::This => observable_entity,
		}
	}
}

#[derive(Event, Debug)]
pub struct SubscribeFor<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	subscriber_entity: SubscriberEntity,
	/// This entity can only be spawned from this events constructors
	subscription_entity: Entity,
	scheduled: bool,
	_phantom_data: PhantomData<O>,
}

impl<O> SubscribeFor<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	/// Be aware that if you can't subscribe to a scheduled observable
	/// with an unscheduled subscribe request
	pub fn unscheduled(
		subscriber_entity: SubscriberEntity,
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
		subscriber_entity: SubscriberEntity,
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
