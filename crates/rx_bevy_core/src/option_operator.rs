use short_type_name::short_type_name;

use crate::{
	ObservableOutput, Observer, ObserverInput, Operation, OperationSubscriber, Operator,
	SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike, Teardown,
};

/// [Operator]s with the same outputs as its inputs can be made optional.
/// If upon subscription the operator was [Some] the subscription will be
/// created with the operator, if it's [None], values will just pass through.
impl<In, InError, Op> Operator for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: 'static,
	InError: 'static,
{
	type Subscriber<Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>>
		= OptionOperatorSubscriber<Op::Subscriber<Destination>, Destination>
	where
		Op::Subscriber<Destination>: Observer;

	fn operator_subscribe<'c, Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination> {
		match self {
			Some(operator) => {
				OptionOperatorSubscriber::Some(operator.operator_subscribe(destination, context))
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

pub enum OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
{
	Some(Sub),
	None(Destination),
}

impl<Sub, Destination> ObserverInput for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
{
	type In = Sub::In;
	type InError = Sub::InError;
}

impl<Sub, Destination> SignalContext for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	type Context = <Destination as SignalContext>::Context;
}

impl<Sub, Destination> Observer for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn next(&mut self, next: Self::In, context: &mut Self::Context) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.next(next, context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.next(next, context)
			}
		}
	}

	fn error(&mut self, error: Self::InError, context: &mut Self::Context) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.error(error, context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.error(error, context)
			}
		}
	}

	fn complete(&mut self, context: &mut Self::Context) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.complete(context)
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.complete(context)
			}
		}
	}

	fn tick(&mut self, tick: crate::Tick, context: &mut Self::Context) {
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

impl<Sub, Destination> SubscriptionLike for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
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

	fn unsubscribe(&mut self, context: &mut <Destination as SignalContext>::Context) {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.unsubscribe(context);
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.unsubscribe(context);
			}
		}
	}

	fn get_unsubscribe_context(&mut self) -> Option<Self::Context> {
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.get_unsubscribe_context()
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.get_unsubscribe_context()
			}
		}
	}
}

impl<Sub, Destination> SubscriptionCollection for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
	Sub: SubscriptionCollection,
	Destination: SubscriptionCollection,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		match self {
			OptionOperatorSubscriber::Some(internal_subscriber) => {
				internal_subscriber.add(subscription, context);
			}
			OptionOperatorSubscriber::None(fallback_subscriber) => {
				fallback_subscriber.add(subscription, context);
			}
		}
	}
}

impl<Sub, Destination> Operation for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	type Destination = Destination;
}

impl<Sub, Destination> Drop for OptionOperatorSubscriber<Sub, Destination>
where
	Sub: OperationSubscriber<
			Destination = Destination,
			Context = <Destination as SignalContext>::Context,
		>,
	Destination: Subscriber<In = Sub::In, InError = Sub::InError>,
	Sub::In: 'static,
	Sub::InError: 'static,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			panic!(
				"Dropped {} without unsubscribing!",
				short_type_name::<Self>()
			)
		}
	}
}
