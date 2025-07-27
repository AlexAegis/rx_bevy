use bevy_ecs::{
	component::{Component, Mutable, StorageType},
	entity::Entity,
};
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator};

use crate::{
	CommandSubscribeExtension, CommandSubscriber, ObservableComponent, ObservableOnInsertContext,
	PipeSubscription, RelativeEntity, SignalBound, Subscribe, SubscriberContext,
	observable_on_insert_hook, observable_on_remove_hook,
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
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	source: RelativeEntity,
	pipe_source_observer_entity: Option<Entity>,
	pipe_source_subscription_entity: Option<Entity>,
	operator: Op,
}

impl<Op> PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(source: RelativeEntity, operator: Op) -> Self {
		Self {
			source,
			operator,
			pipe_source_subscription_entity: None,
			pipe_source_observer_entity: None,
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
	// 	NextOp::In: ObservableSignalBound,
	// 	NextOp::InError: ObservableSignalBound,
	// 	NextOp::Out: ObservableSignalBound,
	// 	NextOp::OutError: ObservableSignalBound,
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
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
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
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type In = Op::In;
	type InError = Op::InError;
}

impl<Op> ObservableOutput for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	type Out = Op::Out;
	type OutError = Op::OutError;
}

impl<Op> ObservableComponent for PipeComponent<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	/// A Subject is also an observer, so if subscriptions to itself were
	/// allowed, an infinite loop would happen
	const CAN_SELF_SUBSCRIBE: bool = false;

	type Subscription = PipeSubscription<Op>;

	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}

	/// The subscription creates a new observer entity, that entity should have the subscriber on it.
	fn on_subscribe(
		&mut self,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
		subscribe_event: &Subscribe<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		let source_observable = subscriber.resolve_relative_entity(&self.source);
		let subscription_entity = subscriber.get_subscription_entity();
		println!(
			"on subscribe pipe {} {}",
			source_observable, subscription_entity
		);
		let source_subscription = {
			let commands = subscriber.commands();

			commands
				.clone_and_retarget_subscription::<Self::Out, Self::OutError, Op::In, Op::InError>(
					subscribe_event,
					source_observable,
					subscription_entity,
				)
		};

		println!("source_subscription {}", source_subscription);

		// let source_subscription = subscription_entity;
		PipeSubscription::<Op>::new(
			source_subscription,
			self.operator.operator_subscribe(subscriber.downgrade()),
		)
	}
}
