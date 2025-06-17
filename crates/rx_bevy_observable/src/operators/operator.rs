use crate::{ObservableOutput, Observer, ObserverInput, Subscription};

pub trait Subscriber: 'static + Observer + Subscription {}
impl<T> Subscriber for T where T: 'static + Observer + Subscription {}

/// An operation is something that does something to its [`Self::Destination`]
pub trait Operation {
	type Destination: Observer;
}

pub trait OperationSubscriber: 'static + Observer + Operation + Subscription {}
impl<T> OperationSubscriber for T where T: 'static + Observer + Operation + Subscription {}

pub trait Operator: ObserverInput + ObservableOutput {
	type Subscriber<Destination:  Subscriber<In = Self::Out, InError = Self::OutError>>:  OperationSubscriber<Destination = Destination, In = Self::In, InError = Self::InError>;

	fn operator_subscribe<Destination: Subscriber<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}
