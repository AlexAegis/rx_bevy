use bevy_input::keyboard::KeyCode;
use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_traits::{
	Observable, ObservableOutput, PrimaryCategoryObservable, SubscriptionContext, SubscriptionData,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

use crate::KeyboardSubscription;

/// A simplistic observable to demonstrate accessing world state from within a subscription
#[derive(Default)]
pub struct KeyboardObservable;

impl ObservableOutput for KeyboardObservable {
	type Out = KeyCode;
	type OutError = ();
}

impl WithSubscriptionContext for KeyboardObservable {
	type Context = BevySubscriptionContextProvider;
}

impl WithPrimaryCategory for KeyboardObservable {
	type PrimaryCategory = PrimaryCategoryObservable;
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
		let subscription = KeyboardSubscription::new(destination.upgrade());
		SubscriptionData::new_from_resource(subscription.into())
	}
}
