#[cfg(feature = "channel_context")]
use crate::ChannelContext;
use crate::{Subscription, UpgradeableObserver};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	#[must_use = "If unused, the subscription will immediately unsubscribe."]
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) -> Subscription;
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}
