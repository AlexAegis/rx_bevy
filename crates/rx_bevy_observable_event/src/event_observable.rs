use std::marker::PhantomData;

use bevy_ecs::{entity::Entity, event::Event};
use rx_bevy_context::RxBevyScheduler;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Never, Observable, SchedulerHandle, Subscriber, UpgradeableObserver};

use crate::EntityEventSubscription;

/// A simplistic observable to demonstrate accessing world state from within a subscription
#[derive(RxObservable)]
#[rx_out(E)]
#[rx_out_error(Never)]
pub struct EventObservable<E>
where
	E: Event + Clone,
{
	observed_entity: Entity,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	_phantom_data: PhantomData<E>,
}

impl<E> EventObservable<E>
where
	E: Event + Clone,
{
	pub fn new(observed_entity: Entity, scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		Self {
			observed_entity,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<E> Observable for EventObservable<E>
where
	E: Event + Clone,
{
	type Subscription<Destination>
		= EntityEventSubscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		EntityEventSubscription::new(
			self.observed_entity,
			destination.upgrade(),
			self.scheduler.clone(),
		)
	}
}
