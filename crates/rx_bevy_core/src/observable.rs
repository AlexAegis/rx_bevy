use crate::{SignalContext, Subscriber, SubscriptionCollection};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	type Subscription: SubscriptionCollection + Default;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>
			+ SubscriptionCollection;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
