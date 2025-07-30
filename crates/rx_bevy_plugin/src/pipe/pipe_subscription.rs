use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{
	ObservableOutput, Observer, ObserverInput, Operation, Operator, SubscriptionLike, Tick,
};

use crate::{
	CommandSubscriber, ObserverSignalPush, RxSubscriber, RxSubscription, SignalBound,
	SubscriberContext,
};

#[cfg(feature = "debug")]
use std::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
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
	fn on_signal(
		&mut self,
		signal: crate::RxSignal<Self::In, Self::InError>,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		#[cfg(feature = "debug")]
		dbg!(signal.clone());

		self.operator_subscriber.push(signal);
		self.operator_subscriber.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
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

	fn on_tick(
		&mut self,
		tick: Tick,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		self.operator_subscriber.tick(tick.clone());
		self.operator_subscriber.write_destination(|destination| {
			destination.forward_buffer(&mut subscriber);
		});
		subscriber.tick(tick);
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
