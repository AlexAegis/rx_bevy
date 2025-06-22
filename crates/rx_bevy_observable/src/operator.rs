use std::ops::Deref;

use crate::{ObservableOutput, Observer, ObserverInput, Subscriber};

/// # [Operator]
///
/// An [Operator] defines its own inputs and output, and a [OperationSubscriber]
/// that defines how those input signals will produce output signals.
pub trait Operator: ObserverInput + ObservableOutput + Clone {
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>: OperationSubscriber<Destination = Destination, In = Self::In, InError = Self::InError>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}

/// An [OperationSubscriber] is a more specialized version of a [Subscriber]
/// used by [Operators]. It's a [Subscriber] that is aware of its Destination
/// because it has constrains on its own outputs.
pub trait OperationSubscriber: Subscriber + Operation {}
impl<T> OperationSubscriber for T where T: Subscriber + Operation {}

/// An operation is something that does something to its [`Self::Destination`]
pub trait Operation {
	type Destination: Observer;
}

impl<T, Target> Operation for T
where
	Target: 'static + Operation,
	T: Deref<Target = Target> + std::ops::DerefMut<Target = Target>,
{
	type Destination = Target::Destination;
}
