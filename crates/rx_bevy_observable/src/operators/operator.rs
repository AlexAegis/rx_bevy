use crate::{ObservableOutput, Observer, ObserverInput, Subscription};

/// A [Subscriber] is an [Observable] that is also a [Subscription], to let it
/// clean itself up upon unsubscribe.
///
/// [Subscriber]s are always owned by something and are never passed as references, hence 'static.
///
/// A struct implementing [Subscriber] should have all their fields as private,
/// as users will never directly interact with a [Subscriber]
pub trait Subscriber: 'static + Observer + Subscription {}
impl<T> Subscriber for T where T: 'static + Observer + Subscription {}

/// An operation is something that does something to its [`Self::Destination`]
/// TODO: Add a get_destination and get_destination_mut methods so subscription can be auto implemented
pub trait Operation {
	type Destination: Observer;
}

/// An [OperationSubscriber] is a more specialized version of a [Subscriber]
/// used by [Operators]. It's a [Subscriber] that is aware of it's Destination
/// because it has constrains on it's own outputs.
pub trait OperationSubscriber: Subscriber + Operation {}
impl<T> OperationSubscriber for T where T: Subscriber + Operation {}

/// An [Operator] defines it's own inputs and output, and an [OperationSubscriber]
/// that defines how those inputs will produce an output.
pub trait Operator: ObserverInput + ObservableOutput {
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>: OperationSubscriber<Destination = Destination, In = Self::In, InError = Self::InError>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}
