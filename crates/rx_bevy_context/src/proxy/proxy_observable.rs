use std::marker::PhantomData;

use bevy_ecs::entity::Entity;

use crate::BevySubscriptionContextProvider;
use rx_core_traits::{
	Observable, ObservableOutput, PrimaryCategoryObservable, SignalBound, SubscriptionContext,
	SubscriptionData, UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

use super::proxy_subscription::ProxySubscription;

/// An observable that sources its events by just subscribing to another
/// entity.
pub struct ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	target_observable_entity: Entity,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	pub fn new(target_observable_entity: Entity) -> Self {
		Self {
			target_observable_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ObservableOutput for ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	type Out = In;
	type OutError = InError;
}

impl<In, InError> WithSubscriptionContext for ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	type Context = BevySubscriptionContextProvider;
}

impl<In, InError> WithPrimaryCategory for ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<In, InError> Observable for ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	/// TODO: Maybe the destination generic should make a comeback
	type Subscription = SubscriptionData<Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>
			+ Send
			+ Sync,
	{
		let subscription = ProxySubscription::new(
			self.target_observable_entity,
			destination.upgrade(),
			context,
		);
		SubscriptionData::new_from_resource(subscription.into())
	}
}
