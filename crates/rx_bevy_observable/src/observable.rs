use crate::{Observer, Subscription};

pub trait ObservableOutput {
	type Out;
	type OutError;
}

pub trait Observable: ObservableOutput {
	type Subscription: Subscription + 'static;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::OutError>>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription;
}
