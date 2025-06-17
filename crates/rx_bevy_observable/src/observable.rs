use crate::{Observer, Subscription};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	type Subscription: Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription;
}
