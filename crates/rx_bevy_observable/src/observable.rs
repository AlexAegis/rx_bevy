use crate::{Observer, Subscription};

pub trait ObservableOutput {
	type Out;
	type OutError;
}

pub trait Observable: ObservableOutput {
	type Subscription: Subscription;

	fn subscribe<Destination: Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription;
}
