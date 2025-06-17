use crate::{ObservableOutput, Observer, ObserverInput, Subscription};

pub trait Subscriber: Observer + Subscription {}
impl<T> Subscriber for T where T: Observer + Subscription {}

/// An operation is something that does something to its [`Self::Destination`]
pub trait Operation {
	type Destination: Observer;
}

pub trait OperationSubscriber: Observer + Operation + Subscription {}
impl<T> OperationSubscriber for T where T: Observer + Operation + Subscription {}

pub trait Operator: ObserverInput + ObservableOutput {
	type Subscriber<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>: OperationSubscriber<Destination = Destination, In = Self::In, InError = Self::InError>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}
