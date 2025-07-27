use bevy_ecs::entity::Entity;
use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Operator, SubscriptionLike, Tick,
};

use crate::{CommandSubscriber, ScheduledSubscription, SignalBound, SubscriberContext};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	source_subscription: Entity,
	operator: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>,
}

impl<Op> PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(
		source_subscription: Entity,
		operator: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>,
	) -> Self {
		Self {
			source_subscription,
			operator,
		}
	}
}

impl<Op> ScheduledSubscription for PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	const SCHEDULED: bool = true;

	fn on_tick(
		&mut self,
		tick: Tick,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		self.tick(tick.clone());
		self.operator.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
		subscriber.tick(tick);
	}

	fn unsubscribe(&mut self, mut subscriber: CommandSubscriber<Self::Out, Self::OutError>) {
		self.operator.unsubscribe();
		// Drain the operator in case something produced something during teardown
		self.operator.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
		subscriber.unsubscribe();
		subscriber
			.commands()
			.entity(self.source_subscription)
			.despawn();
	}
}

impl<Op> ObserverInput for PipeSubscription<Op>
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

impl<Op> ObservableOutput for PipeSubscription<Op>
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

impl<Op> Observer for PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: SignalBound,
	Op::InError: SignalBound,
	Op::Out: SignalBound,
	Op::OutError: SignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		println!("NEXT PIPE");
		self.operator.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.operator.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.operator.complete();
	}

	#[inline]
	fn tick(&mut self, tick: rx_bevy_observable::Tick) {
		self.operator.tick(tick);
	}
}
