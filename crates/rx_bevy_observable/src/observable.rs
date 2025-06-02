use crate::{Observer, Subscription};

pub trait Observable {
	type Out;
	type Error;

	type Subscription: Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out, Error = Self::Error>>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription;
}
