use bevy_ecs::{
	component::{Component, Mutable, StorageType},
	entity::Entity,
	observer::Trigger,
	system::{Commands, Query},
};
use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator};

use crate::{
	CommandSubscriber, DebugBound, ObservableComponent, ObservableOnInsertContext,
	ObservableSignalBound, PipeSubscription, RxSignal, SubscriptionEntityContext,
	WithSubscribeObserverReference, observable_on_insert_hook, observable_on_remove_hook,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	subscribe_observer_entity: Option<Entity>,
	operator: Op,
}

impl<Op> PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new() {}

	pub fn pipe(self) {
		todo!("impl piping logic, similar to the original pipe struct!")
	}
}

impl<Op> Component for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	const STORAGE_TYPE: StorageType = StorageType::Table;
	type Mutability = Mutable;

	fn register_component_hooks(hooks: &mut bevy_ecs::component::ComponentHooks) {
		hooks.on_insert(observable_on_insert_hook::<Self>);
		hooks.on_remove(observable_on_remove_hook::<Self>);
	}
}

impl<Op> ObserverInput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type In = Op::In;
	type InError = Op::InError;
}

impl<Op> ObservableOutput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Op> WithSubscribeObserverReference for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	fn get_subscribe_observer_entity(&self) -> Option<Entity> {
		self.subscribe_observer_entity
	}

	fn set_subscribe_observer_entity(
		&mut self,
		subscribe_observer_entity: Entity,
	) -> Option<Entity> {
		self.subscribe_observer_entity
			.replace(subscribe_observer_entity)
	}
}

impl<Op> ObservableComponent for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscription = PipeSubscription<Op>;

	fn on_insert(&mut self, context: ObservableOnInsertContext) {
		context
			.commands
			.entity(context.observable_entity)
			.observe(pipe_next_observer::<Op>);
	}

	/// The subscription creates a new observer entity, that entity should have the subscriber on it.
	fn on_subscribe(
		&mut self,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		let static_subscriber = subscriber.downgrade();
		PipeSubscription::<Op>::new(self.operator.operator_subscribe(static_subscriber))
	}
}

fn pipe_next_observer<Op>(
	trigger: Trigger<RxSignal<Op::In, Op::InError>>,
	mut subject_query: Query<(&PipeComponent<Op>,)>,
	mut commands: Commands,
) where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriptionEntityContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	let Ok((pipe,)) = subject_query.get_mut(trigger.target()) else {
		return;
	};

	//commands.trigger_targets(
	//	trigger.event().clone(),
	//	subscription.get_subscriber_entities(),
	//);
}
