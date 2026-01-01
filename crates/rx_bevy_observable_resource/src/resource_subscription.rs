use std::marker::PhantomData;

use bevy_ecs::resource::Resource;
use rx_bevy_context::RxBevyScheduler;
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Observer, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionData, SubscriptionLike, WorkCancellationId, WorkResult,
};

use crate::observable::ResourceObservableOptions;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	#[teardown]
	teardown: SubscriptionData,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	cancellation_id: WorkCancellationId,
	_phantom_data: PhantomData<(R, Reader)>,
}

impl<R, Reader, Destination> ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber,
{
	pub fn new(
		destination: Destination,
		reader: Reader,
		options: ResourceObservableOptions,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let shared_destination = SharedSubscriber::new(destination);
		let subscription_scheduler = scheduler.clone();

		let mut shared_destination_clone = shared_destination.clone();
		let mut scheduler_lock = scheduler.lock();
		let cancellation_id = scheduler_lock.generate_cancellation_id();
		let mut resource_existed_in_the_previous_tick = false;

		scheduler_lock.schedule_continuous_work(
			move |_tick, context| {
				let resource_option = context.deferred_world.get_resource::<R>();
				let is_changed = context.deferred_world.is_resource_changed::<R>();
				let is_added = {
					let resource_exists_this_tick = resource_option.is_some();
					let is_added =
						!resource_existed_in_the_previous_tick && resource_exists_this_tick;
					resource_existed_in_the_previous_tick = resource_exists_this_tick;
					is_added
				};

				// is_changed is always true when is_added is true
				let is_changed_condition = options.trigger_on_is_changed && is_changed && !is_added;
				let is_added_condition = options.trigger_on_is_added && is_added;

				if (is_changed_condition || is_added_condition)
					&& let Some(resource) = resource_option
				{
					let next = (reader)(resource);
					match next {
						Ok(next) => shared_destination_clone.next(next),
						Err(error) => shared_destination_clone.error(error),
					}
				}

				WorkResult::Pending
			},
			cancellation_id,
		);

		Self {
			shared_destination,
			scheduler: subscription_scheduler,
			teardown: SubscriptionData::default(),
			cancellation_id,
			_phantom_data: PhantomData,
		}
	}
}

impl<R, Reader, Destination> SubscriptionLike for ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Result<Destination::In, Destination::InError> + Clone + Send + Sync,
	Destination: 'static + Subscriber,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.shared_destination.unsubscribe();
			self.teardown.unsubscribe();

			self.scheduler.lock().cancel(self.cancellation_id);
		}
	}
}
