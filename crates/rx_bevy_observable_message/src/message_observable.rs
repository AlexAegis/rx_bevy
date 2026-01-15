use std::marker::PhantomData;

use bevy_ecs::event::Event;
use derive_where::derive_where;
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::{
	Never, Observable, PhantomInvariant, SchedulerHandle, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::MessageSubscription;

#[derive_where(Default)]
#[derive(RxObservable)]
#[rx_out(M)]
#[rx_out_error(Never)]
pub struct MessageObservable<M>
where
	M: Event + Clone, // TODO(bevy-0.17): use the message trait
{
	scheduler: SchedulerHandle<RxBevyScheduler>,
	_phantom_data: PhantomInvariant<M>,
}

impl<M> MessageObservable<M>
where
	M: Event + Clone,
{
	pub fn new(scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		Self {
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<M> Observable for MessageObservable<M>
where
	M: Event + Clone,
{
	type Subscription<Destination>
		= MessageSubscription<Destination>
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
		MessageSubscription::new(destination.upgrade(), self.scheduler.clone())
	}
}
