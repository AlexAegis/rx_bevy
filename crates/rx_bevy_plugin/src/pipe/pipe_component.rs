use bevy_ecs::{
	component::{Component, Mutable, StorageType},
	entity::Entity,
	name::Name,
	observer::Trigger,
	system::{Commands, Query},
};
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator};

use crate::{
	CommandSubscriber, ObservableComponent, ObservableOnInsertContext, ObservableSignalBound,
	PipeSubscription, RelativeEntity, RxSignal, Subscribe, SubscriberContext,
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
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	source: RelativeEntity,
	subscribe_observer_entity: Option<Entity>,
	pipe_source_observer_entity: Option<Entity>,
	pipe_source_subscription_entity: Option<Entity>,
	operator: Op,
}

impl<Op> PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(source: RelativeEntity, operator: Op) -> Self {
		Self {
			source,
			operator,
			pipe_source_subscription_entity: None,
			pipe_source_observer_entity: None,
			subscribe_observer_entity: None,
		}
	}

	// #[inline]
	// pub fn pipe<NextOp>(self, operator: NextOp) -> PipeComponent<CompositeOperator<Op, NextOp>>
	// where
	// 	NextOp: 'static
	// 		+ Operator<
	// 			In = <Self as ObservableOutput>::Out,
	// 			InError = <Self as ObservableOutput>::OutError,
	// 		>
	// 		+ Send
	// 		+ Sync
	// 		+ DebugBound,
	// 	NextOp::In: Send + Sync + ObservableSignalBound,
	// 	NextOp::InError: Send + Sync + ObservableSignalBound,
	// 	NextOp::Out: Send + Sync + ObservableSignalBound,
	// 	NextOp::OutError: Send + Sync + ObservableSignalBound,
	// 	NextOp::Subscriber<SubscriptionEntityContext<NextOp::Out, NextOp::OutError>>:
	// 		Send + Sync + DebugBound,
	// 	<Op as Operator>::Subscriber<
	// 		<NextOp as Operator>::Subscriber<
	// 			SubscriptionEntityContext<
	// 				<NextOp as ObservableOutput>::Out,
	// 				<NextOp as ObservableOutput>::OutError,
	// 			>,
	// 		>,
	// 	>: Send + Sync + DebugBound,
	// {
	// 	PipeComponent::<CompositeOperator<Op, NextOp>>::new(
	// 		self.source,
	// 		CompositeOperator::new(self.operator, operator),
	// 	)
	// }
}

impl<Op> Component for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
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
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
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
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
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
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
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
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscription = PipeSubscription<Op>;

	fn on_insert(&mut self, context: ObservableOnInsertContext) {
		let source_observable = self.source.this_or(context.observable_entity);
		// TODO: FINISH, On insert, only setup should happen, the source subscription should happen on subscribe, so each
		// TODO: subscription to the pipe has a new instance of it too.
		let (event, subscription_entity) = Subscribe::<Self::Out, Self::OutError>::unscheduled(
			RelativeEntity::Other(source_observable),
			context.commands,
		);

		let pipe_source_observer_entity = context
			.commands
			.spawn((
				Name::new(format!(
					"Observer Pipe Subscriber {}",
					context.observable_entity
				)),
				bevy_ecs::prelude::Observer::new(pipe_next_observer::<Self, Op>)
					.with_entity(self.source.this_or(context.observable_entity)),
			))
			.id();

		self.pipe_source_observer_entity = Some(pipe_source_observer_entity);
	}

	/// The subscription creates a new observer entity, that entity should have the subscriber on it.
	fn on_subscribe(
		&mut self,
		subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		let static_subscriber = subscriber.downgrade();
		PipeSubscription::<Op>::new(self.operator.operator_subscribe(static_subscriber))
	}
}

fn pipe_next_observer<O, Op>(
	trigger: Trigger<RxSignal<Op::In, Op::InError>>,
	mut subject_query: Query<(&PipeComponent<Op>,)>,
	mut commands: Commands,
) where
	O: ObservableComponent + Send + Sync,
	O::Out: Clone + ObservableSignalBound,
	O::OutError: Clone + ObservableSignalBound,
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	let Ok((pipe,)) = subject_query.get_mut(trigger.target()) else {
		return;
	};

	//commands.trigger_targets(
	//	trigger.event().clone(),
	//	subscription.get_subscriber_entities(),
	//);
}
