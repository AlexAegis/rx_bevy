use bevy::{ecs::component::Mutable, prelude::*};
use bevy_ecs::{component::HookContext, world::DeferredWorld};
use derive_where::derive_where;
use rx_bevy::ObservableOutput;
use short_type_name::short_type_name;
use std::{fmt::Debug, marker::PhantomData};

use crate::{
	DebugBound, ObservableSignalBound, RxTick, ScheduledSubscription, SubscriptionComponent,
	Subscriptions,
};

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
pub trait ObservableComponent:
	ObservableOutput + Component<Mutability = Mutable> + DebugBound
where
	Self::Out: Send + Sync + DebugBound,
	Self::OutError: Send + Sync + DebugBound,
{
	const CAN_SELF_SUBSCRIBE: bool;

	/// If the Observable does not need any scheduling, use [NonScheduledSubscription]
	/// Otherwise implement a [ScheduledSubscription] that can emit events when
	/// ticked by an [RxScheduler].
	type ScheduledSubscription: ScheduledSubscription<Out = Self::Out, OutError = Self::OutError>
		+ Send
		+ Sync;

	/// Should return the entity reference to the entity that observes [Subscribe]
	/// events for this observable
	/// TODO(relationship-one-on-one): Refactor once one-on-one relationships are a thing
	fn get_subscribe_observer_entity(&self) -> Option<Entity>;

	/// Returns the previous observer entity, if exists.
	/// (Implement as `.replace` on the stored `Option<Entity>`)
	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity>;

	fn on_insert(&mut self, context: ObservableOnInsertContext);

	fn on_subscribe(
		&mut self,
		context: ObservableOnSubscribeContext,
	) -> Self::ScheduledSubscription;
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
	const TICKABLE: bool = false;

	fn on_event(&mut self, _event: crate::RxNext<Self::Out>, _context: SubscriptionOnTickContext) {
		unreachable!()
	}

	fn on_tick(&mut self, _event: &RxTick, _context: SubscriptionOnTickContext) {
		unreachable!()
	}
}

#[derive_where(Debug)]
pub struct ObservableOnInsertContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
}

#[derive_where(Debug)]
pub struct ObservableOnSubscribeContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

#[derive_where(Debug)]
pub struct SubscriptionOnTickContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

// TODO: So that you can just .next stuff instead of emitting values by hand
//impl Observer for SubscriptionOnTickContext {}

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
		return;
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
		return;
	}

	// Spawn (soon-to-be) Subscription entity
	let subscription_entity = commands.spawn_empty().id();

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
		let scheduled_subscription =
			observable_component.on_subscribe(ObservableOnSubscribeContext {
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
		if O::ScheduledSubscription::TICKABLE {
			// It's observing itself!
			subscription_entity_commands.insert(
				bevy::ecs::prelude::Observer::new(subscription_tick_observer::<O>)
					.with_entity(subscription_entity),
			);
		};
	}
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarkerComponent;

/// This is what would drive an "intervalObserver" ticking a subscriber,
/// that will decide if it should next something to its subscribers or not
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
		let context =
			subscription.into_subscription_on_tick_context(&mut commands, trigger.target());
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
	pub subscriber_entity: SubscriberEntity,
	pub _phantom_data: PhantomData<O>,
}

impl<O> SubscribeFor<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	pub fn new(subscriber_entity: SubscriberEntity) -> Self {
		Self {
			subscriber_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<O> From<SubscriberEntity> for SubscribeFor<O>
where
	O: ObservableComponent,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	fn from(subscriber_entity: SubscriberEntity) -> Self {
		Self::new(subscriber_entity)
	}
}
