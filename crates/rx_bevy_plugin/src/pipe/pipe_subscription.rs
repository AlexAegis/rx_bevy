use bevy_ecs::{
	entity::Entity,
	observer::Trigger,
	system::{Commands, Query, SystemParam},
	world::Mut,
};
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Operator, SubscriptionLike,
};

use crate::{
	CommandSubscriber, ObserverSignalPush, RxNext, RxSubscriber, RxSubscription, RxTick,
	SignalBound, SubscriberContext, Subscription, SubscriptionChannelHandlerRegistrationContext,
	SubscriptionSignalDestination,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct PipeSubscriber<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	operator_subscriber: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>,
}

impl<Op> PipeSubscriber<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(operator: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>) -> Self {
		Self {
			operator_subscriber: operator,
		}
	}
}

impl<Op> RxSubscriber for PipeSubscriber<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	fn register_subscriber_channel_handlers<'a, 'w, 's>(
		&mut self,
		mut handlers: crate::SubscriberChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		handlers.register_next_handler(pipe_on_next_hook::<Op>);
	}
}

impl<Op> RxSubscription for PipeSubscriber<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	const SCHEDULED: bool = true;

	fn register_subscription_channel_handlers<'a, 'w, 's>(
		&mut self,
		mut handlers: SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		handlers.register_tick_handler(pipe_on_tick_hook::<Op>);
	}

	fn unsubscribe(&mut self, mut subscriber: CommandSubscriber<Self::Out, Self::OutError>) {
		self.operator_subscriber.unsubscribe();
		// Drain the operator in case something produced something during teardown
		self.operator_subscriber.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
		subscriber.unsubscribe();
		// subscriber
		// 	.commands()
		// 	.entity(self.source_subscription)
		// 	.despawn();
	}
}

#[derive(SystemParam)]
pub struct RxDestination<'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	commands: Commands<'w, 's>,
	destination_query: Query<'w, 's, &'static SubscriptionSignalDestination<Sub>>,
}

impl<'w, 's, Sub> RxDestination<'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn get_subscriber_of<'a>(
		&'a mut self,
		subscription_entity: Entity,
	) -> CommandSubscriber<
		'a,
		'w,
		's,
		<Sub as ObservableOutput>::Out,
		<Sub as ObservableOutput>::OutError,
	> {
		let destination = self
			.destination_query
			.get_mut(subscription_entity)
			.unwrap_or_else(|_| {
				panic!(
					"A subscription must have a destination, but was not found on {}",
					subscription_entity
				)
			});
		destination
			.get_subscription_entity_context(subscription_entity)
			.upgrade(&mut self.commands)
	}
}

#[derive(SystemParam)]
pub struct RxContextSub<'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	commands: Commands<'w, 's>,
	subscription_query: Query<'w, 's, &'static mut Subscription<Sub>>,
	destination_query: Query<'w, 's, &'static SubscriptionSignalDestination<Sub>>,
}

impl<'w, 's, Sub> RxContextSub<'w, 's, Sub>
where
	Sub: RxSubscription,
	Sub::Out: SignalBound,
	Sub::OutError: SignalBound,
{
	pub fn get_subscription<'a>(
		&'a mut self,
		subscription_entity: Entity,
	) -> Mut<'a, Subscription<Sub>> {
		self.subscription_query
			.get_mut(subscription_entity)
			.unwrap_or_else(|_| {
				panic!(
					"Subscription component {} was not found on {}",
					short_type_name::short_type_name::<Sub>(),
					subscription_entity
				)
			})
	}

	pub fn get_destination<'a>(
		&'a mut self,
		subscription_entity: Entity,
	) -> CommandSubscriber<
		'a,
		'w,
		's,
		<Sub as ObservableOutput>::Out,
		<Sub as ObservableOutput>::OutError,
	> {
		let destination = self.destination_query.get_mut(subscription_entity).unwrap();
		destination
			.get_subscription_entity_context(subscription_entity)
			.upgrade(&mut self.commands)
	}
}

fn pipe_on_tick_hook<Op>(
	trigger: Trigger<RxTick>,
	mut context: RxContextSub<PipeSubscriber<Op>>,
	mut destination: RxDestination<PipeSubscriber<Op>>,
) where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	let mut subscription = context.get_subscription(trigger.target());
	let mut subscriber = destination.get_subscriber_of(trigger.target());
	subscription
		.operator_subscriber
		.tick((**trigger.event()).clone());
	subscription
		.operator_subscriber
		.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
}

fn pipe_on_next_hook<Op>(
	trigger: Trigger<RxNext<Op::In>>,
	mut context: RxContextSub<PipeSubscriber<Op>>,
	mut destination: RxDestination<PipeSubscriber<Op>>,
) where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	let mut subscription = context.get_subscription(trigger.target());
	let mut subscriber = destination.get_subscriber_of(trigger.target());
	subscription
		.operator_subscriber
		.push(trigger.event().clone());
	subscription
		.operator_subscriber
		.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
}

impl<Op> ObserverInput for PipeSubscriber<Op>
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

impl<Op> ObservableOutput for PipeSubscriber<Op>
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
