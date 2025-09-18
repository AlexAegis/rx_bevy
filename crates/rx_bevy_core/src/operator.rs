use crate::{ObservableOutput, Observer, ObserverInput, SignalContext, Subscriber};

/// # [Operator]
///
/// An [Operator] defines its own inputs and output, and a [OperationSubscriber]
/// that defines how those input signals will produce output signals.
pub trait Operator: ObserverInput + ObservableOutput + Clone {
	// TODO: Should be into destination context so the context can be downgraded along the operators
	type Subscriber<Destination>: OperationSubscriber<
			Destination = Destination,
			In = Self::In,
			InError = Self::InError,
			Context = Destination::Context,
		>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>;
}

/// An [OperationSubscriber] is a more specialized version of a [Subscriber]
/// used by [Operators]. It's a [Subscriber] that is aware of its Destination
/// because it has constrains on its own outputs.
pub trait OperationSubscriber: 'static + Subscriber + Operation {}

impl<T> OperationSubscriber for T where T: 'static + Subscriber + Operation {}

/// An operation is something that does something to its [`Self::Destination`]
/// TODO: Reevaluate if this trait is still needed or not.
pub trait Operation {
	type Destination: Observer;
}
