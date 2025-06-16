use crate::{ObservableOutput, Observer, ObserverInput, Subscription};

// TODO: Not sure if Subscriber should require ObservableOutput, since it's already in Destination
pub trait Subscriber: Observer + ObserverInput + ObservableOutput + Subscription {
	type Destination: Observer<In = Self::Out, InError = Self::OutError>;
}

pub trait Operator: ObserverInput + ObservableOutput {
	type Subscriber<D: Observer<In = Self::Out, InError = Self::OutError>>: Subscriber<
			Destination = D,
			In = Self::In,
			InError = Self::InError,
			Out = Self::Out,
			OutError = Self::OutError,
		>;

	fn operator_subscribe<
		Destination: 'static
			+ Observer<
				In = <Self as ObservableOutput>::Out,
				InError = <Self as ObservableOutput>::OutError,
			>,
	>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>;
}
