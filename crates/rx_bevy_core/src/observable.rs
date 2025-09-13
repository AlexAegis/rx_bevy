use crate::{SignalContext, Subscriber, SubscriptionCollection, SubscriptionLike};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable<'c>: ObservableOutput + SignalContext {
	type Subscription: Default + SubscriptionLike + SubscriptionCollection<'c>
	where
		Self::Context: 'c;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination: Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self as SignalContext>::Context,
			>;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
