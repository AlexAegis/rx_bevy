use crate::{Subscriber, SubscriptionCollection, SubscriptionLike};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable<Subscription>: ObservableOutput
where
	Subscription: SubscriptionLike + SubscriptionCollection,
{
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Subscription::Context,
	) -> Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Subscription::Context>;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
