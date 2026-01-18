use std::marker::PhantomData;

use bevy_ecs::resource::Resource;
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::{
	Never, Observable, PhantomInvariant, SchedulerHandle, Signal, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::{ResourceSubscription, observable::ResourceObservableOptions};

/// # [ResourceObservable]
///
/// The `ResourceObservable` call a "reader" function on an observable every
/// time it is added or mutated, emitting the result to subscribers.
///
/// ## Options
///
/// - `trigger_on_is_added`: Emit also when the resource was just added.
///   (Default: true)
/// - `trigger_on_is_changed`: Emit on each tick where the resource was accessed
///   mutably, except when the resource was just added.
///   (Default: true)
#[derive(RxObservable)]
#[rx_out(Out)]
#[rx_out_error(Never)]
pub struct ResourceObservable<R, Reader, Out>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Out + Clone + Send + Sync,
	Out: Signal,
{
	reader: Reader,
	options: ResourceObservableOptions,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	_phantom_data: PhantomInvariant<R>,
}

impl<R, Reader, Out> ResourceObservable<R, Reader, Out>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Out + Clone + Send + Sync,
	Out: Signal,
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

impl<R, Reader, Out> Observable for ResourceObservable<R, Reader, Out>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Out + Clone + Send + Sync,
	Out: Signal,
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
