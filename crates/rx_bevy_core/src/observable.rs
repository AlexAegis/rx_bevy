use crate::{SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	// TODO: YES, because of context restraining ->
	// TODO: Is it better to lock every observable to a single concrete subscription, or should it be on the generic?
	type Subscription: SubscriptionLike + SubscriptionCollection;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Subscription as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
