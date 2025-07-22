use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query},
	world::DeferredWorld,
};
use bevy_log::{debug, error, trace, warn};
use derive_where::derive_where;

use rx_bevy_observable::{ObservableOutput, SubscriptionLike};
use short_type_name::short_type_name;

use crate::{
	CommandSubscriber, DebugBound, EntityContext, ObservableSignalBound, RxTick,
	ScheduledSubscription, Subscribe, SubscriberContext, SubscriptionComponent, Subscriptions,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

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
	///  TODO: This should really need SubscriptionLike but for that the command-less subscribercontext would also need to impl observer
	type Subscription: ScheduledSubscription<Out = Self::Out, OutError = Self::OutError>
		+ Send
		+ Sync;

	fn on_insert(&mut self, context: ObservableOnInsertContext);

	/// The subscriber received can immediately be used to push events into
	/// using it's Observer interface (`.next`, `.error`, `.complete`)
	/// To achieve this, it has a reference to [Commands] in it, which makes it
	/// impossible to store in a subscription. To do that, you need to `downgrade`
	/// the subscriber, which just returns everything in it minus the commands reference.
	/// Later (in another frame) it can be upgraded to a subscriber with a new reference
	/// to [Commands].
	fn on_subscribe(
		&mut self,
		subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription;
}

/// TODO: While this is required for all ObservableComponents, it's a separate trait to be the auto-implementable by a macro.
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

#[derive_where(Debug)]
pub struct ObservableOnInsertContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
}

/// This on_insert hook sets up the observable so it can spawn new subscriptions
/// upon receiving [Subscribe] events.
pub fn observable_on_insert_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
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
		trace!(
			"setting up subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);

		commands
			.spawn((
				ChildOf(observable_entity), // Purely for organizational purposes in debug views like WorldInspector
				Name::new(format!(
					"Observer (Observable Subscribe) - {}({}) ",
					short_type_name::<O>(),
					observable_entity
				)),
				Observer::new(on_subscribe::<O>).with_entity(observable_entity),
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

	trace!(
		"setting up subscribe observer for {}({}) finished",
		short_type_name::<O>(),
		observable_entity
	);
}

/// To achieve a one-on-one relationship, the observer that observes [Subscribe] events
/// is despawned when the observable component is removed
pub fn observable_on_remove_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
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

fn on_subscribe<O>(
	trigger: Trigger<Subscribe<O::Out, O::OutError>>,
	mut observable_component_query: Query<(&mut O, Option<&mut Subscriptions<O>>)>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: ObservableSignalBound,
	O::OutError: ObservableSignalBound,
{
	let observable_entity = trigger.target();
	debug!("on_subscribe {}", observable_entity);
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

	if !O::Subscription::SCHEDULED && trigger.event().is_scheduled() {
		error!(
			"Tried to subscribe to an unscheduled observable with a scheduled Subscription! {}({})",
			short_type_name::<O>(),
			observable_entity
		);
		return; // Err(SubscribeError::ScheduledSubscribeOnUnscheduledObservable.into());
	}

	// Get the pre-spawned scheduled Subscription entity
	let subscription_entity = trigger.event().get_subscription_entity();

	// Initialize the Subscriptions component on the observable
	if let Some(mut subscriptions) = existing_subscriptions_component {
		// In case the Entity contains more than one observable with the same signals
		if !subscriptions.contains(subscription_entity) {
			subscriptions.push(subscription_entity);
		}
	} else {
		// Technically a required component, but [ObservableComponent] is a trait, so it's inserted lazily
		commands
			.entity(observable_entity)
			.insert(Subscriptions::<O>::new(subscription_entity));
	}

	{
		let context = SubscriberContext::new(EntityContext {
			source_entity: observable_entity,
			destination_entity,
			subscription_entity,
		});

		let scheduled_subscription =
			observable_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscription_entity_commands = commands.entity(subscription_entity);

		// TODO: If we're subscribing to multiple Observables, completion requires some merge-like logic, count how many observables we have, to know how many completions we need.
		subscription_entity_commands.insert_if_new((
			Name::new(format!(
				"Subscription<{}, {}> for [{}]",
				short_type_name::<O::Out>(),
				short_type_name::<O::OutError>(),
				observable_entity
			)),
			SubscriptionComponent::<O>::new(
				observable_entity,
				destination_entity,
				scheduled_subscription,
			),
		));

		if O::Subscription::SCHEDULED {
			subscription_entity_commands.insert_if_new((
				SubscriptionMarker,
				Observer::new(subscription_tick_observer::<O>).with_entity(subscription_entity), // It's observing itself!
			));
		};
	}
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionMarker;

/// This is what would drive an "intervalObserver" ticking a subscriber,
/// that will decide if it should next something to its subscribers or not
///
/// Notice how the schedule is not present. The [RxScheduler] plugin will
/// query based on the Schedule but the Subscription itself does not have to be
/// aware of the Schedule it runs on.
fn subscription_tick_observer<O>(
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
		let subscriber = subscription
			.get_subscription_entity_context(trigger.target())
			.upgrade(&mut commands);
		subscription.tick(trigger.event(), subscriber);
	}
}
