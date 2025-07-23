use rx_bevy_common_bounds::DebugBound;
use rx_bevy_observable::{ObservableOutput, ObserverInput, Operator, SubscriptionLike, Tick};

use crate::{CommandSubscriber, ObservableSignalBound, ScheduledSubscription, SubscriberContext};

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
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	operator: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>,
}

impl<Op> PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	pub fn new(operator: Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>) -> Self {
		Self { operator }
	}
}

impl<Op> ScheduledSubscription for PipeSubscription<Op>
where
	Op: 'static + Operator + Send + Sync + DebugBound,
	Op::In: Send + Sync + ObservableSignalBound,
	Op::InError: Send + Sync + ObservableSignalBound,
	Op::Out: Send + Sync + ObservableSignalBound,
	Op::OutError: Send + Sync + ObservableSignalBound,
	Op::Subscriber<SubscriberContext<Op::Out, Op::OutError>>: Send + Sync + DebugBound,
{
	const SCHEDULED: bool = true;

	fn on_tick(
		&mut self,
		event: &Tick,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		//let destination = self.operator.read(|destination| {
		//	//destination.
		//
		//	subscriber.next();
		//});
		println!("pipe subscription ticked! {event:?}");
		// TODO: Figure out how to tick a pipes subscription, upstream changes will be needed!
	}

	fn unsubscribe(&mut self, mut subscriber: CommandSubscriber<Self::Out, Self::OutError>) {
		println!("pipe subscription unsubbed!");
		subscriber.unsubscribe();
	}
}

impl<Op> ObserverInput for PipeSubscription<Op>
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

impl<Op> ObservableOutput for PipeSubscription<Op>
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
