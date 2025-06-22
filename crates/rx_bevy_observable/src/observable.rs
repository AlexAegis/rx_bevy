use crate::{Subscription, UpgradeableObserver};

pub trait ObservableOutput {
	type Out: 'static;
	type OutError: 'static;
}

pub trait Observable: ObservableOutput {
	#[cfg_attr(feature = "inline_subscribe", inline)]
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription;
}
