use crate::{ObservableOutput, Observer, ObserverInput, Subscription};

// TODO: Not sure if Subscriber should require ObservableOutput, since it's already in Destination
pub trait Subscriber: Observer + ObserverInput + ObservableOutput + Subscription {
	type Destination: Observer<In = Self::Out, InError = Self::OutError>;
}

pub trait Operator: ObserverInput + ObservableOutput {
	type Subscriber<Destination: Observer<In = Self::Out, InError = Self::OutError>>: Subscriber<
			Destination = Destination,
			In = Self::In,
			InError = Self::InError,
			Out = Destination::In,
			OutError = Destination::InError,
		>;

	fn operator_subscribe<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}
