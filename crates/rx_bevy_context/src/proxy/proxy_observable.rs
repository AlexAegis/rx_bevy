use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_core_macro_observable_derive::RxObservable;

use crate::BevySubscriptionContextProvider;
use rx_core_traits::{
	Observable, SignalBound, Subscriber, SubscriptionContext, UpgradeableObserver,
};

use super::proxy_subscription::ProxySubscription;

/// An observable that sources its events by just subscribing to another
/// entity.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(In)]
#[rx_out_error(InError)]
#[rx_context(BevySubscriptionContextProvider)]
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

impl<In, InError> Observable for ProxyObservable<In, InError>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
{
	type Subscription<Destination>
		= ProxySubscription<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ UpgradeableObserver<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		ProxySubscription::new(
			self.target_observable_entity,
			destination.upgrade(),
			context,
		)
	}
}
