use bevy_ecs::{
	component::{Component, HookContext, Mutable},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	query::Without,
	system::{Commands, Query},
	world::DeferredWorld,
};
use bevy_log::{debug, trace, warn};

use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput, Tick};
use short_type_name::short_type_name;

use crate::{
	CommandSubscribeExtension, CommandSubscriber, EntityContext, ObservableOnInsertContext,
	OperatorSubscribeObserverOf, RelativeEntity, RxSignal, RxSubscriber, SignalBound, Subscribe,
	SubscriberContext, SubscriberSignalObserverRef, SubscriptionComponent, Subscriptions,
};

use std::any::TypeId;

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
	ObserverInput + ObservableOutput + Component<Mutability = Mutable> + DebugBound
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

	fn on_insert(&mut self, context: ObservableOnInsertContext);

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
	let observable_entity = hook_context.entity;

	// This is the observer that processes [Subscribe] events for this specific observable.
	// It will be despawned when the observable is removed.
	{
		let mut commands = deferred_world.commands();
		debug!(
			"setting up subscribe observer for {}({})",
			short_type_name::<Op>(),
			observable_entity
		);

		let _ = commands
			.spawn((
				OperatorSubscribeObserverOf::<Op>::new(observable_entity),
				Observer::new(on_operator_subscribe::<Op>).with_entity(observable_entity),
				// TODO: Having this here is unnecessary and is causing a warning on despawn because of the double relationship. I'll leave this here for now just so the inspector is a little more organized until that too has a convenient method to register relationships
				ChildOf(observable_entity), // For organizational purposes in debug views like WorldInspector
				Name::new(format!(
					"Observer (Subscribe) - {}({}) ",
					short_type_name::<Op>(),
					observable_entity
				)),
			))
			.id();
	};

	// Calling the on_insert hook on the observable
	{
		let (mut entities, mut commands) = deferred_world.entities_and_commands();
		let mut observable_entity_mut = entities.get_mut(observable_entity).unwrap();

		let mut component = observable_entity_mut.get_mut::<Op>().unwrap();

		component.on_insert(ObservableOnInsertContext {
			observable_entity,
			commands: &mut commands,
		});
	}

	debug!(
		"setting up subscribe observer for {}({}) finished",
		short_type_name::<Op>(),
		observable_entity
	);
}

fn on_operator_subscribe<Op>(
	trigger: Trigger<Subscribe<Op::Out, Op::OutError>>,
	mut observable_component_query: Query<(&mut Op, Option<&mut Subscriptions<Op::Subscriber>>)>,
	mut commands: Commands,
	name_query: Query<&Name>,
) where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	let observable_entity = trigger.target();
	println!(
		"on_subscribe {} {:?}",
		observable_entity,
		name_query.get(observable_entity).unwrap()
	);

	let Ok((mut operator_component, existing_subscriptions_component)) =
		observable_component_query.get_mut(observable_entity)
	else {
		warn!(
			"Tried to subscribe to {} but it does not exist on {}",
			short_type_name::<Op>(),
			observable_entity
		);
		return; // Err(SubscribeError::NotAnObservable.into());
	};
	let destination_entity = trigger.get_subscriber_entity_or_this(observable_entity);

	// Operators may not subscribe to the entity they are on if their input and
	// output types match as that would just feed it into itself
	if observable_entity == destination_entity && TypeId::of::<Op::In>() == TypeId::of::<Op::Out>()
	{
		warn!(
			"Tried to subscribe to itself when it is disallowed! {}({})",
			short_type_name::<Op>(),
			observable_entity
		);
		return; // Err(SubscribeError::SelfSubscribeDisallowed.into());
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
				.insert(Subscriptions::<Op::Subscriber>::new(subscription_entity));
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
			operator_component.on_subscribe(context.upgrade(&mut commands));

		let mut subscription_entity_commands = commands.entity(subscription_entity);

		subscription_entity_commands.insert_if_new((
			Name::new(format!(
				"Subscription<{}, {}> for [{}]",
				short_type_name::<Op::Out>(),
				short_type_name::<Op::OutError>(),
				observable_entity
			)),
			SubscriptionComponent::<Op::Subscriber>::new(
				observable_entity,
				destination_entity,
				scheduled_subscription,
			),
		));

		/// TODO: Check if this could also observe ticks here through an enum, as for regular observables that was only prevented by the scheduler
		subscription_entity_commands.insert_if_new((
			Observer::new(operator_subscription_tick_observer::<Op>)
				.with_entity(subscription_entity), // It's observing itself!
		));
	}

	// Operator Subscription Chain setup
	{
		let source_observable_entity = operator_component.get_source().or_this(observable_entity);

		// TODO: CONTINUE FROM HERE!!!!!!!!!!!!!
		// TODO: This needs to be .add-ed to the current subscription to form a teardown chain. The subscriptions relation could do that, on remove that would despawn all anyway,
		// TODO: that may require unifying collecting subscriptions on Out, OutError as now it's either an operator or a normal sub, and there isn't really a difference
		let source_subscription_entity = commands
			.subscribe_with_schedule_of::<Op::Out, Op::OutError, Op::In, Op::InError>(
				source_observable_entity,
				subscription_entity,
				trigger.event(),
			);
		// TODO: Add a subscriptions to this subscription, and a subref to the

		println!(
			"op sub for chain sub {}",
			short_type_name::<Op::Subscriber>()
		);
		commands.entity(source_observable_entity);

		dbg!(source_subscription_entity);
	}

	// Setting up signal observer
	{
		commands.spawn((
			Name::new(format!(
				"Operator Signal Observer <{}, {}, {}, {}> for [{}]",
				short_type_name::<Op::In>(),
				short_type_name::<Op::InError>(),
				short_type_name::<Op::Out>(),
				short_type_name::<Op::OutError>(),
				subscription_entity
			)),
			Observer::new(operator_subscription_signal_observer::<Op>)
				.with_entity(subscription_entity),
		));
	}
}

fn operator_subscription_tick_observer<Op>(
	trigger: Trigger<Tick>,
	mut subscription_query: Query<
		&mut SubscriptionComponent<Op::Subscriber>,
		Without<SubscriberSignalObserverRef<Op>>, // Subscribers aren't directly ticked, they are ticked by other subscriptions
	>,
	mut commands: Commands,
) where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
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

fn operator_subscription_signal_observer<Op>(
	trigger: Trigger<RxSignal<Op::In, Op::InError>>,
	mut subscription_query: Query<
		&mut SubscriptionComponent<Op::Subscriber>,
		Without<SubscriberSignalObserverRef<Op>>, // Subscribers aren't directly ticked, they are ticked by other subscriptions
	>,
	mut commands: Commands,
) where
	Op: OperatorComponent + Send + Sync,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
{
	#[cfg(feature = "debug")]
	trace!(
		"operator_subscription_signal_observer {:?}",
		trigger.event()
	);

	if let Ok(mut subscription) = subscription_query.get_mut(trigger.target()) {
		let subscriber = subscription
			.get_subscription_entity_context(trigger.target())
			.upgrade(&mut commands);

		if let Some(s) = &mut subscription.scheduled_subscription {
			s.on_signal(trigger.event().clone(), subscriber);
		};
	}
}
