use std::marker::PhantomData;

use bevy_input::keyboard::KeyCode;
use rx_bevy_context::{BevySubscriptionContextProvider, EntitySubscriptionContextAccessProvider};
use rx_core_traits::{
	Observable, ObservableOutput, Subscriber, SubscriptionData,
	prelude::{SubscriptionContext, WithSubscriptionContext},
};

use crate::KeyboardSubscription;

/// A simplistic observable to demonstrate accessing world state from within a subscription
pub struct KeyboardObservable<ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<ContextAccess> ObservableOutput for KeyboardObservable<ContextAccess>
where
	ContextAccess: EntitySubscriptionContextAccessProvider,
{
	type Out = KeyCode;
	type OutError = ();
}

impl<ContextAccess> WithSubscriptionContext for KeyboardObservable<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<ContextAccess> Observable for KeyboardObservable<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	/// TODO: Maybe the destination generic should make a comeback
	type Subscription = SubscriptionData<Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let subscription = KeyboardSubscription::<Destination, ContextAccess>::new(destination);
		SubscriptionData::new_with_teardown(subscription.into())
	}
}
