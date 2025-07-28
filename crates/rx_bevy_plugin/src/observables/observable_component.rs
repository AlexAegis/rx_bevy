use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	entity::Entity,
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	query::Without,
	system::{Commands, Query},
	world::DeferredWorld,
};
use bevy_log::{debug, error, trace, warn};
use derive_where::derive_where;

use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, Tick};
use short_type_name::short_type_name;

use crate::{
	CommandSubscriber, EntityContext, RxSubscription, SignalBound, Subscribe, SubscribeObserverOf,
	SubscribeObserverRef, SubscriberContext, SubscriberSignalObserverRef, SubscriptionComponent,
	Subscriptions,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
///
/// Reflection: As many Operators are generic over their closures, which do not
/// have a type_path it is impossible to require reflection over observables.
pub trait ObservableComponent:
	ObservableOutput + Component<Mutability = Mutable> + DebugBound
where
	Self::Out: SignalBound,
	Self::OutError: SignalBound,
{
	const CAN_SELF_SUBSCRIBE: bool;

	/// If the Observable does not need any scheduling, use [NonScheduledSubscription]
	/// Otherwise implement a [ScheduledSubscription] that can emit events when
	/// ticked by an [RxScheduler].
	type Subscription: RxSubscription<Out = Self::Out, OutError = Self::OutError> + Send + Sync;

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

#[derive_where(Debug)]
pub struct ObservableOnInsertContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
}

/// This on_insert hook sets up the observable so it can spawn new subscriptions
/// upon receiving [Subscribe] events.
/// This is key to decouple the request to create a subscription from the
/// observable components actual type.
pub fn observable_on_insert_hook<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	let observable_entity = hook_context.entity;

	// This is the observer that processes [Subscribe] events for this specific observable.
	// It will be despawned when the observable is removed.
	{
		let mut commands = deferred_world.commands();
		trace!(
			"setting up subscribe observer for {}({})",
			short_type_name::<O>(),
			observable_entity
		);

		commands.spawn((
			SubscribeObserverOf::<O>::new(observable_entity),
			Observer::new(on_subscribe::<O>).with_entity(observable_entity),
			// TODO: Having this here is unnecessary and is causing a warning on despawn because of the double relationship. I'll leave this here for now just so the inspector is a little more organized until that too has a convenient method to register relationships
			ChildOf(observable_entity), // For organizational purposes in debug views like WorldInspector
			Name::new(format!(
				"Observer (Subscribe) - {}({}) ",
				short_type_name::<O>(),
				observable_entity
			)),
		));
	};

	// Calling the on_insert hook on the observable
	{
		let (mut entities, mut commands) = deferred_world.entities_and_commands();
		let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();

		let mut component = observable_entity_mut.get_mut::<O>().unwrap();

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

pub fn observable_on_remove_hook<O>(mut _deferred_world: DeferredWorld, _hook_context: HookContext)
where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	// TODO: Unsubscribe all subscriptions
}

fn on_subscribe<O>(
	trigger: Trigger<Subscribe<O::Out, O::OutError>>,
	mut observable_component_query: Query<(&mut O, Option<&mut Subscriptions<O>>)>,
	mut commands: Commands,
	name_query: Query<&Name>,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound,
	O::OutError: SignalBound,
{
	let observable_entity = trigger.target();
	debug!(
		"on_subscribe {} {:?}",
		observable_entity,
		name_query.get(observable_entity).unwrap()
	);
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

	// Relationship management
	let subscription_entity = {
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

		subscription_entity
	};

	{
		let context = SubscriberContext::new(EntityContext {
			source_entity: observable_entity,
			destination_entity,
			subscription_entity,
		});

		let scheduled_subscription =
			observable_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscription_entity_commands = commands.entity(subscription_entity);

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

		// TODO: Another child entity is needed for the subscriber that then can OBSERVE signals to form subscription chains, child because it needs to be despawned together. Probably will be used by switchmap

		if O::Subscription::SCHEDULED {
			subscription_entity_commands.insert_if_new((
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
///
/// This only ticks direct subscriptions to observables, and not operators.
/// These direct subscriptions will forward the tick to the operator subscribers
/// to ensure correct event order.
fn subscription_tick_observer<O>(
	trigger: Trigger<Tick>,
	mut subscription_query: Query<&mut SubscriptionComponent<O>>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: SignalBound + Clone,
	O::OutError: SignalBound,
{
	#[cfg(feature = "debug")]
	trace!("subscription_tick_observer {:?}", trigger.event());

	if let Ok(mut subscription) = subscription_query.get_mut(trigger.target()) {
		let subscriber = subscription
			.get_subscription_entity_context(trigger.target())
			.upgrade(&mut commands);

		subscription.tick(trigger.event().clone(), subscriber);
	}
}
