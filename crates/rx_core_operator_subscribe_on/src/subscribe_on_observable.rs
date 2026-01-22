use core::marker::PhantomData;
use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use rx_core_common::{
	Observable, PhantomInvariant, Scheduler, SchedulerHandle, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::subscribe_on_subscription::SubscribeOnSubscription;

#[derive(RxObservable, Clone)]
#[rx_out(Source::Out)]
#[rx_out_error(Source::OutError)]
pub struct SubscribeOnObservable<Source, S>
where
	Source: 'static + Observable,
	S: Scheduler,
{
	source: Arc<Mutex<Source>>,
	delay: Duration,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<Source>,
}

impl<Source, S> SubscribeOnObservable<Source, S>
where
	Source: 'static + Observable,
	S: Scheduler,
{
	pub fn new(source: Source, delay: Duration, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			source: Arc::new(Mutex::new(source)),
			delay,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<Source, S> Observable for SubscribeOnObservable<Source, S>
where
	Source: 'static + Observable + Send + Sync,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscription<Destination>
		= SubscribeOnSubscription<Destination, Source>
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
		SubscribeOnSubscription::new(
			destination.upgrade(),
			self.source.clone(),
			self.delay,
			self.scheduler.clone(),
		)
	}
}
