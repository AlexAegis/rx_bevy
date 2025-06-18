use crate::{Observer, Subscriber, Subscription};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	type Subscriber<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>: Subscriber<In = Self::Out, InError = Self::OutError>;

	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<Destination: 'static + Observer<In = Self::Out, InError = Self::OutError>>(
		&mut self,
		destination: Destination,
	) -> Subscription<Self::Subscriber<Destination>>;
}
