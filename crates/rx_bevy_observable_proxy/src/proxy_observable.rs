use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_common::RxBevyScheduler;
use rx_core_macro_observable_derive::RxObservable;

use rx_core_common::{
	Observable, PhantomInvariant, SchedulerHandle, Signal, Subscriber, UpgradeableObserver,
};

use super::proxy_subscription::ProxySubscription;

/// An observable that sources its events by just subscribing to another
/// entity.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct ProxyObservable<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	target_observable_entity: Entity,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> ProxyObservable<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	pub fn new(
		target_observable_entity: Entity,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		Self {
			target_observable_entity,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Observable for ProxyObservable<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	type Subscription<Destination>
		= ProxySubscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		ProxySubscription::new(
			self.target_observable_entity,
			destination.upgrade(),
			self.scheduler.clone(),
		)
	}
}
