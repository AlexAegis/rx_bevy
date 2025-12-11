use std::marker::PhantomData;

use bevy_ecs::resource::Resource;
use rx_bevy_context::RxBevyScheduler;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, SchedulerHandle, Signal, Subscriber, UpgradeableObserver};

use crate::{ResourceSubscription, observable::ResourceObservableOptions};

#[derive(RxObservable)]
#[rx_out(Out)]
#[rx_out_error(OutError)]
pub struct ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: Signal,
	OutError: Signal,
{
	reader: Reader,
	options: ResourceObservableOptions,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	_phantom_data: PhantomData<R>,
}

impl<R, Reader, Out, OutError> ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: Signal,
	OutError: Signal,
{
	pub fn new(
		reader: Reader,
		options: ResourceObservableOptions,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		Self {
			reader,
			options,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<R, Reader, Out, OutError> Observable for ResourceObservable<R, Reader, Out, OutError>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Out, OutError> + Clone + Send + Sync,
	Out: Signal,
	OutError: Signal,
{
	type Subscription<Destination>
		= ResourceSubscription<R, Reader, Destination>
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
		ResourceSubscription::new(
			destination.upgrade(),
			self.reader.clone(),
			self.options.clone(),
			self.scheduler.clone(),
		)
	}
}
