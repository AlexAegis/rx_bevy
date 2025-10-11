use crate::{ObservableOutput, ObserverInput, Operator, OptionSubscriber, SignalBound, Subscriber};

/// [Operator]s with the same outputs as its inputs can be made optional.
///
/// If upon subscription, the operator was [Some] the subscription will be
/// created with the operator, if it's [None], values will just pass through.
impl<In, InError, Op> Operator for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type Context = Op::Context;
	type Subscriber<Destination>
		= OptionSubscriber<Op::Subscriber<Destination>, Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
		Op::Subscriber<Destination>: Subscriber;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		match self {
			Some(operator) => {
				OptionSubscriber::Some(operator.operator_subscribe(destination, context))
			}
			None => OptionSubscriber::None(destination),
		}
	}
}

impl<In, InError, Op> ObserverInput for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Op> ObservableOutput for Option<Op>
where
	Op: Operator<In = In, InError = InError, Out = In, OutError = InError>,
	In: SignalBound,
	InError: SignalBound,
{
	type Out = In;
	type OutError = InError;
}
