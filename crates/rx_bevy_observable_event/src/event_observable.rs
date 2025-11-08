use std::{fmt::Debug, marker::PhantomData};

use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_context::BevySubscriptionContextProvider;
use rx_core_traits::{
	Observable, ObservableOutput, PrimaryCategoryObservable, SubscriptionContext, SubscriptionData,
	UpgradeableObserver, WithPrimaryCategory, WithSubscriptionContext,
};

use crate::EntityEventSubscription;

/// A simplistic observable to demonstrate accessing world state from within a subscription
pub struct EventObservable<E>
where
	E: Event + Clone,
{
	observed_entity: Entity,
	_phantom_data: PhantomData<E>,
}

impl<E> EventObservable<E>
where
	E: Event + Clone,
{
	pub fn new(observed_entity: Entity) -> Self {
		Self {
			observed_entity,
			_phantom_data: PhantomData,
		}
	}
}

impl<E> ObservableOutput for EventObservable<E>
where
	E: Event + Clone,
{
	type Out = E;
	type OutError = ();
}

impl<E> WithSubscriptionContext for EventObservable<E>
where
	E: Event + Clone,
{
	type Context = BevySubscriptionContextProvider;
}

impl<E> WithPrimaryCategory for EventObservable<E>
where
	E: Event + Clone,
{
	type PrimaryCategory = PrimaryCategoryObservable;
}

impl<E> Observable for EventObservable<E>
where
	E: Event + Clone + Debug,
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
		let subscription = EntityEventSubscription::<E, _>::new(
			self.observed_entity,
			destination.upgrade(),
			context,
		);
		SubscriptionData::new_from_resource(subscription.into())
	}
}
