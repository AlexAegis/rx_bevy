use bevy_input::keyboard::KeyCode;
use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Never, Observable, SubscriptionContext, SubscriptionData, UpgradeableObserver,
};

use crate::{KeyboardObservableOptions, KeyboardSubscription};

/// A simplistic observable to demonstrate accessing world state from within a subscription
#[derive(RxObservable, Default)]
#[rx_out(KeyCode)]
#[rx_out_error(Never)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct KeyboardObservable {
	options: KeyboardObservableOptions,
}

impl KeyboardObservable {
	pub fn new(options: KeyboardObservableOptions) -> Self {
		Self { options }
	}
}

impl Observable for KeyboardObservable {
	/// TODO: Maybe the destination generic should make a comeback
	type Subscription = SubscriptionData<Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let subscription = KeyboardSubscription::new(destination.upgrade(), self.options.clone());
		SubscriptionData::new_from_resource(subscription.into())
	}
}
