use crate::{DropContext, ObservableOutput, ObserverInput, Subscriber, SubscriptionCollection};

/// # [Operator]
///
/// An [Operator] defines its own inputs and output, and a Subscriber
/// that defines how those input signals will produce output signals.
///
/// Operators choose a single Context type for the whole subscription chain
/// they participate in. Downstream and upstream must agree on this Context.
pub trait Operator: ObserverInput + ObservableOutput {
	type Context: DropContext;

	type Subscriber<Destination>: 'static
		+ Subscriber<In = Self::In, InError = Self::InError, Context = Self::Context>
		+ SubscriptionCollection
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ SubscriptionCollection;
}
