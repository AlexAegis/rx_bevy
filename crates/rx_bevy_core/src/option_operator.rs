use crate::{
	ObservableOutput, Observer, ObserverInput, Operation, OperationSubscriber, Operator,
	Subscriber, SubscriptionLike,
};

#[cfg(feature = "channel_context")]
use crate::ChannelContext;

/// [Operator]s with the same outputs as its inputs can be made optional.
/// If upon subscription the operator was [Some] the subscription will be
/// created with the operator, if it's [None], values will just pass through.
impl<In, InError, Op> Operator for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: 'static,
	InError: 'static,
{
	type Subscriber<D: 'static + Subscriber<In = Self::Out, InError = Self::OutError>> =
		OptionOperatorSubscriber<Op::Subscriber<D>, D>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination> {
		match self {
			Some(operator) => {
				OptionOperatorSubscriber::Some(operator.operator_subscribe(destination))
			}
			None => OptionOperatorSubscriber::None(destination),
		}
	}
}

impl<In, InError, Op> ObserverInput for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Op> ObservableOutput for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: 'static,
	InError: 'static,
{
	type Out = In;
	type OutError = InError;
}

pub enum OptionOperatorSubscriber<OpSub, Destination>
where
	OpSub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = OpSub::In, InError = OpSub::InError>,
{
	Some(OpSub),
	None(Destination),
}

impl<Sub, Destination> ObserverInput for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
{
	type In = Sub::In;
	type InError = Sub::InError;
}

#[cfg(feature = "channel_context")]
impl<Sub, Destination> Observer for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn next(&mut self, next: Self::In, context: &mut ChannelContext) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.next(next, context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.next(next, context)
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut ChannelContext) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.error(error, context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.error(error, context)
			}
		}
	}

	fn complete(&mut self, context: &mut ChannelContext) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.complete(context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.complete(context)
			}
		}
	}

	#[cfg(feature = "tick")]
	fn tick(&mut self, tick: crate::Tick, context: &mut ChannelContext) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.tick(tick, context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.tick(tick, context)
			}
		}
	}
}

#[cfg(not(feature = "channel_context"))]
impl<Sub, Destination> Observer for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn next(&mut self, next: Self::In) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => internal_subscriber.next(next),
			OptionOperatorSubscriber::None(fallback_subscriber) => fallback_subscriber.next(next),
		}
	}

	fn error(&mut self, error: Self::InError) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => internal_subscriber.error(error),
			OptionOperatorSubscriber::None(fallback_subscriber) => fallback_subscriber.error(error),
		}
	}

	fn complete(&mut self) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => internal_subscriber.complete(),
			OptionOperatorSubscriber::None(fallback_subscriber) => fallback_subscriber.complete(),
		}
	}

	#[cfg(feature = "tick")]
	fn tick(
		&mut self,
		tick: crate::Tick,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => internal_subscriber.tick(tick),
			OptionOperatorSubscriber::None(fallback_subscriber) => fallback_subscriber.tick(tick),
		}
	}
}

impl<Sub, Destination> Operation for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
{
	type Destination = Destination;

	/// Let's you check the shared observer for the duration of the callback
	fn read_destination<F>(&self, reader: F)
	where
		F: Fn(&Self::Destination),
	{
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.read_destination(reader)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => reader(fallback_subscriber),
		}
	}

	/// Let's you check the shared observer for the duration of the callback
	fn write_destination<F>(&mut self, mut writer: F)
	where
		F: FnMut(&mut Self::Destination),
	{
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.write_destination(writer)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => writer(fallback_subscriber),
		}
	}
}

impl<Sub, Destination> SubscriptionLike for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn is_closed(&self) -> bool {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => internal_subscriber.is_closed(),
			OptionOperatorSubscriber::None(fallback_subscriber) => fallback_subscriber.is_closed(),
		}
	}

	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				#[cfg(feature = "channel_context")]
				internal_subscriber.unsubscribe(context);
				#[cfg(not(feature = "channel_context"))]
				internal_subscriber.unsubscribe();
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				#[cfg(feature = "channel_context")]
				fallback_subscriber.unsubscribe(context);
				#[cfg(not(feature = "channel_context"))]
				fallback_subscriber.unsubscribe();
			}
		}
	}

	fn add(
		&mut self,
		subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				#[cfg(feature = "channel_context")]
				internal_subscriber.add(subscription, context);
				#[cfg(not(feature = "channel_context"))]
				internal_subscriber.add(subscription);
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				#[cfg(feature = "channel_context")]
				fallback_subscriber.add(subscription, context);
				#[cfg(not(feature = "channel_context"))]
				fallback_subscriber.add(subscription);
			}
		}
	}
}

impl<Sub, Destination> Drop for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<Destination = Destination>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn drop(&mut self) {
		#[cfg(not(feature = "channel_context"))]
		self.unsubscribe();

		#[cfg(feature = "channel_context")]
		if !self.is_closed() {
			panic!(
				"Dropped {} without unsubscribing first while feature 'channel_context' is enabled!",
				short_type_name::short_type_name::<Self>()
			)
		}
	}
}
