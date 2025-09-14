use crate::{SignalContext, Subscriber, SubscriptionCollection};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput + SignalContext {
	type Subscription: SubscriptionCollection<Context = Self::Context> + Default;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut Self::Context,
	) -> Self::Subscription
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
