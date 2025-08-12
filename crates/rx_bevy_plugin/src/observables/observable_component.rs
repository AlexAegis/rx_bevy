use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	entity::Entity,
	error::BevyError,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query, SystemParam},
	world::DeferredWorld,
};
use bevy_log::{debug, trace};
use derive_where::derive_where;
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, Tick};
use short_type_name::short_type_name;

use crate::{
	CommandSubscriber, DeferredWorldObservableCallOnInsertExtension,
	DeferredWorldObservableSpawnObservableSubscribeObserverExtension, EntityContext, RxChannel,
	RxChannelTick, RxSubscription, RxTick, SignalBound, Subscribe, SubscribeError,
	SubscriberContext, SubscriberInstanceOf, SubscriberInstances, Subscription,
	SubscriptionChannelHandlerRef, SubscriptionChannelHandlerRegistrationContext,
	SubscriptionMarker, SubscriptionSignalDestination,
};

#[cfg(feature = "reflect")]
use crate::DeferredWorldObservableRegisterSubscriptionTypesExtension;

/// TODO: CONTINUE Fix Subject, probably needs it's on on_subject_subscribe handler and SubjectComponent types
/// TODO: Check if even needed, once SubjectComponent is fixed
pub trait OnInsertSubHook {
	fn on_insert(&mut self, context: ObservableOnInsertContext);
}

/// Since the nature of a Subscription is very different in the context of an
/// ECS, where there are no long term references, the nature of an Observable
/// also changes.
///
/// Reflection: As many Operators are generic over their closures, which do not
/// have a type_path it is impossible to require reflection over observables.
pub trait ObservableComponent:
	ObservableOutput + Component<Mutability = Mutable> + OnInsertSubHook + DebugBound
where
	Self::Out: SignalBound,
	Self::OutError: SignalBound,
{
	const CAN_SELF_SUBSCRIBE: bool;

	/// If the Observable does not need any scheduling, use [NonScheduledSubscription]
	/// Otherwise implement a [ScheduledSubscription] that can emit events when
	/// ticked by an [RxScheduler].
	type Subscription: RxSubscription<Out = Self::Out, OutError = Self::OutError> + Send + Sync;

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
	#[cfg(feature = "reflect")]
	deferred_world.register_subscription_types::<O::Subscription>();

	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<O>(&mut deferred_world);

	deferred_world.spawn_observable_subscribe_observer::<O>(hook_context.entity);
	deferred_world.call_on_insert_hook::<O>(hook_context.entity);
}

/// Removes the subscriptions for this observable or operators subscriber,
/// causing them to unsubscribe
pub fn observable_on_remove_hook<Sub>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Sub: 'static + RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<SubscriberInstances<Sub>>();
}

pub(crate) fn on_observable_subscribe<O>(
	trigger: Trigger<Subscribe<O::Out, O::OutError>>,
	mut observable_component_query: Query<&mut O>,
	mut commands: Commands,
	name_query: Query<&Name>,
) -> Result<(), BevyError>
where
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
	let Ok(mut observable_component) = observable_component_query.get_mut(observable_entity) else {
		return Err(
			SubscribeError::NotAnObservable(short_type_name::<O>(), observable_entity).into(),
		);
	};
	let destination_entity = trigger.get_destination_or_this(observable_entity);

	// Observables that re-emit everything they observe should not be able to
	// subscribe to themselves as that would cause an infinite loop
	if !O::CAN_SELF_SUBSCRIBE && observable_entity == destination_entity {
		return Err(SubscribeError::SelfSubscribeDisallowed(
			short_type_name::<O>(),
			observable_entity,
		)
		.into());
	}

	if O::Subscription::SCHEDULED && !trigger.event().is_scheduled() {
		return Err(SubscribeError::UnscheduledSubscribeOnScheduledObservable(
			short_type_name::<O>(),
			observable_entity,
		)
		.into());
	}

	if !O::Subscription::SCHEDULED && trigger.event().is_scheduled() {
		return Err(SubscribeError::ScheduledSubscribeOnUnscheduledObservable(
			short_type_name::<O>(),
			observable_entity,
		)
		.into());
	}

	let subscription_entity = trigger.event().get_subscription_entity();

	{
		let context = SubscriberContext::new(EntityContext {
			destination_entity,
			subscription_entity,
		});

		let mut spawned_subscription =
			observable_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscription_hooks =
			SubscriptionChannelHandlerRegistrationContext::<O::Subscription>::new(
				subscription_entity,
				&mut commands,
			);
		spawned_subscription.register_channel_handlers(&mut subscription_hooks);

		let mut subscription_entity_commands = commands.entity(subscription_entity);

		subscription_entity_commands.insert((
			Name::new(format!(
				"Subscription<{}, {}> for [{}]",
				short_type_name::<O::Out>(),
				short_type_name::<O::OutError>(),
				observable_entity
			)),
			SubscriptionMarker,
			Subscription::<O::Subscription>::new(spawned_subscription),
			SubscriberInstanceOf::<O::Subscription>::new(observable_entity),
			SubscriptionSignalDestination::<O::Subscription>::new(destination_entity),
		));

		if O::Subscription::SCHEDULED {
			// The [SubscriptionSchedule] component was already inserted into this entity
			subscription_entity_commands.insert((
				Observer::new(subscription_tick_observer::<O::Subscription>)
					.with_entity(subscription_entity), // It's observing itself!
			));
		};
	}

	Ok(())
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
/// TODO: Extend this so it observes all channels, next,error,complete,unsub,tick
pub(crate) fn subscription_tick_observer<Sub>(
	trigger: Trigger<Tick>,
	rx_context: RxChannelDestination<Sub, RxChannelTick>,
	mut commands: Commands,
) where
	Sub: RxSubscription,
	Sub::Out: SignalBound + Clone,
	Sub::OutError: SignalBound,
{
	#[cfg(feature = "debug")]
	trace!("subscription_tick_observer {:?}", trigger.event());

	let destination = rx_context.get_next_destination_with_on_tick_hook(trigger.target());
	commands.trigger_targets(RxTick(trigger.event().clone()), destination);
}

#[derive(SystemParam)]
pub struct RxChannelDestination<'w, 's, Sub, Channel>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	commands: Commands<'w, 's>,
	subscription_query: Query<
		'w,
		's,
		(
			Entity,
			&'static SubscriptionSignalDestination<Sub>,
			Option<&'static SubscriptionChannelHandlerRef<Channel, Sub>>,
		),
	>,
}

impl<'w, 's, Sub, Channel> RxChannelDestination<'w, 's, Sub, Channel>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
	Channel: RxChannel,
{
	/// Returns the next entity in the destination chain with a hook for
	/// the signal, or the final destination
	pub fn get_next_destination_with_on_tick_hook(&self, from: Entity) -> Entity {
		let mut cursor = from;
		let target: Entity;

		loop {
			if let Ok((entity, destination, hook)) = self.subscription_query.get(cursor) {
				cursor = destination.get_destination();
				if hook.is_some() {
					target = entity;
					break;
				}
			} else {
				target = cursor;
				break;
			}
		}

		target
	}
}
