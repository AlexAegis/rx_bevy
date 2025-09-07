use crate::{SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	type Subscription: 'static + Default + SubscriptionLike + SubscriptionCollection;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<'c, Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as SignalContext>::Context<'c>,
	) -> Self::Subscription
	where
		Destination: Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context<'c> = <Self::Subscription as SignalContext>::Context<'c>,
			>;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
