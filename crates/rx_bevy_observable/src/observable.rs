use crate::{Observer, Subscription};

pub trait Observable {
	type Out;

	type Subscription: Subscription;

	fn subscribe<Destination: 'static + Observer<In = Self::Out>>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription;
}
