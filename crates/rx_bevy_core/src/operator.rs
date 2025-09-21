use crate::{ObservableOutput, ObserverInput, SignalContext, Subscriber};

/// # [Operator]
///
/// An [Operator] defines its own inputs and output, and a [OperationSubscriber]
/// that defines how those input signals will produce output signals.
pub trait Operator: ObserverInput + ObservableOutput + Clone {
	// TODO: Should be into destination context so the context can be downgraded along the operators
	type Subscriber<Destination>: Subscriber<In = Self::In, InError = Self::InError, Context = Destination::Context>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Subscriber<Destination> as SignalContext>::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: Subscriber<In = Self::Out, InError = Self::OutError>;
}
