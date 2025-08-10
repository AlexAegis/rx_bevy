use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	error::BevyError,
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	query::Without,
	system::{Commands, Query},
	world::DeferredWorld,
};
use bevy_log::trace;
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput};
use short_type_name::short_type_name;
use std::any::TypeId;

use crate::{
	CommandSubscribeExtension, CommandSubscriber, DeferredWorldObservableCallOnInsertExtension,
	DeferredWorldObservableSpawnOperatorSubscribeObserverExtension, EntityContext, OnInsertSubHook,
	RelativeEntity, RxSignal, RxSubscriber, SignalBound, Subscribe, SubscribeError,
	SubscriberContext, SubscriberHooks, SubscriberInstanceOf, SubscriberSignalObserverRef,
	Subscription, SubscriptionSignalDestination, subscription_tick_observer,
};

#[cfg(feature = "reflect")]
use crate::DeferredWorldObservableRegisterSubscriptionTypesExtension;

/// Unlike an [ObservableComponent], an [OperatorComponent] differs in what its
/// "subscription" does. Upon subscribe, an operator returns an [RxSubscriber]
/// which is a Subscription that is also an [Observer] of signals. The difference
/// is that an operator can react to input signals, and not just ticks. It also
/// always requires a source observable that will produce said input signals.
///
/// Operators usually produce output signals based on the input signals, but they
/// can implement more complex behavior, for example repeating an input signal
/// for 5 more frames on each tick after the signal was received. Or start with
/// some signals upon subscription.
///
pub trait OperatorComponent:
	ObserverInput + ObservableOutput + Component<Mutability = Mutable> + OnInsertSubHook + DebugBound
where
	Self::In: SignalBound,
	Self::InError: SignalBound,
	Self::Out: SignalBound,
	Self::OutError: SignalBound,
{
	type Subscriber: RxSubscriber<
			In = Self::In,
			InError = Self::InError,
			Out = Self::Out,
			OutError = Self::OutError,
		> + Send
		+ Sync;

	fn get_source(&self) -> RelativeEntity;

	fn on_subscribe(
		&mut self,
		subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscriber;
}

/// This on_insert hook sets up the observable so it can spawn new subscriptions
/// upon receiving [Subscribe] events.
/// This is key to decouple the request to create a subscription from the
/// observable components actual type.
pub fn operator_on_insert_hook<Op>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	#[cfg(feature = "debug")]
	crate::register_operator_debug_systems::<Op>(&mut deferred_world);

	#[cfg(feature = "reflect")]
	deferred_world.register_subscription_types::<Op::Subscriber>();

	// This is the observer that processes [Subscribe] events for this specific observable.
	// It will be despawned when the observable is removed.
	deferred_world.spawn_operator_subscribe_observer::<Op>(hook_context.entity);

	deferred_world.call_on_insert_hook::<Op>(hook_context.entity);
}
pub(crate) fn on_operator_subscribe<Op>(
	trigger: Trigger<Subscribe<Op::Out, Op::OutError>>,
	mut observable_component_query: Query<&mut Op>,
	mut commands: Commands,
) -> Result<(), BevyError>
where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	let operator_definition_entity = trigger.target();

	let Ok(mut operator_component) = observable_component_query.get_mut(operator_definition_entity)
	else {
		return Err(SubscribeError::NotAnObservable(
			short_type_name::<Op>(),
			operator_definition_entity,
		)
		.into());
	};
	let destination_entity = trigger.get_destination_or_this(operator_definition_entity);

	// Operators may not subscribe to the entity they are on if their input and
	// output types match as that would just feed it into itself
	if operator_definition_entity == destination_entity
		&& TypeId::of::<Op::In>() == TypeId::of::<Op::Out>()
	{
		return Err(SubscribeError::SelfSubscribeDisallowed(
			short_type_name::<Op>(),
			operator_definition_entity,
		)
		.into());
	}

	let subscription_entity = trigger.event().get_subscription_entity();

	// Setting up Subscription
	{
		let context = SubscriberContext::new(EntityContext {
			destination_entity,
			subscription_entity,
		});

		let mut spawned_subscriber =
			operator_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscriber_hooks = SubscriberHooks::<Op::Subscriber>::default();
		spawned_subscriber.register_hooks(&mut subscriber_hooks.upgrade(&mut commands));

		dbg!(&subscriber_hooks);

		let mut subscription_entity_commands = commands.entity(subscription_entity);

		subscription_entity_commands.insert((
			Name::new(format!(
				"Subscription<{}, {}> for [{}]",
				short_type_name::<Op::Out>(),
				short_type_name::<Op::OutError>(),
				operator_definition_entity
			)),
			Subscription::<Op::Subscriber>::new(spawned_subscriber),
			subscriber_hooks,
			SubscriberInstanceOf::<Op::Subscriber>::new(operator_definition_entity),
			SubscriptionSignalDestination::<Op::Subscriber>::new(destination_entity),
		));

		subscription_entity_commands.insert((
			Observer::new(subscription_tick_observer::<Op::Subscriber>)
				.with_entity(subscription_entity), // It's observing itself!
		));
	}

	// No longer needed, operator implementation spawns these through hook registrations
	//// Setting up signal observer
	//{
	//	commands.spawn((
	//		Name::new(format!(
	//			"Operator Signal Observer <{}, {}, {}, {}> for [{}]",
	//			short_type_name::<Op::In>(),
	//			short_type_name::<Op::InError>(),
	//			short_type_name::<Op::Out>(),
	//			short_type_name::<Op::OutError>(),
	//			subscription_entity
	//		)),
	//		ChildOf(subscription_entity),
	//		Observer::new(operator_subscription_signal_observer::<Op>)
	//			.with_entity(subscription_entity),
	//	));
	//}

	// Operator Subscription Chain setup
	{
		let source_observable_entity = operator_component
			.get_source()
			.or_this(operator_definition_entity);

		// TODO: Check if this needs to be added AND FORWARDED to the first subscription entity, or the linked_spawn attr is enough (works currently, needs some more testing)
		let _source_subscription_entity = commands
			.subscribe_with_schedule_of::<Op::Out, Op::OutError, Op::In, Op::InError>(
				source_observable_entity,
				subscription_entity,
				trigger.event(),
			);
	}

	Ok(())
}

//fn operator_subscription_signal_observer<Op>(
//	trigger: Trigger<RxSignal<Op::In, Op::InError>>,
//	mut subscription_query: Query<
//		(
//			&SubscriptionSignalDestination<Op::Subscriber>,
//			&mut Subscription<Op::Subscriber>,
//		),
//		Without<SubscriberSignalObserverRef<Op>>, // Subscribers aren't directly ticked, they are ticked by other subscriptions
//	>,
//	mut commands: Commands,
//) where
//	Op: OperatorComponent + Send + Sync,
//	Op::In: SignalBound,
//	Op::InError: SignalBound,
//	Op::Out: SignalBound,
//	Op::OutError: SignalBound,
//{
//	#[cfg(feature = "debug")]
//	trace!(
//		"operator_subscription_signal_observer {:?}",
//		trigger.event()
//	);
//
//	if let Ok((signal_destination, mut subscription)) = subscription_query.get_mut(trigger.target())
//	{
//		let subscriber = signal_destination
//			.get_subscription_entity_context(trigger.target())
//			.upgrade(&mut commands);
//
//		subscription.on_signal(trigger.event().clone(), subscriber);
//	}
//}
