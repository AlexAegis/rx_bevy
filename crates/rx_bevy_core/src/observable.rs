use crate::{SignalContext, SubscriptionLike, UpgradeableObserver};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	type Subscription: 'static + SubscriptionLike;

	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<
		Destination: 'static
			+ UpgradeableObserver<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	>(
		&mut self,
		destination: Destination,
		context: &mut Destination::Context,
	) -> Self::Subscription;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
