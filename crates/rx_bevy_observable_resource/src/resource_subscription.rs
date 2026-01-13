use std::marker::PhantomData;

use bevy_ecs::resource::Resource;
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::{
	RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionLike, Teardown, TeardownCollectionExtension, WorkResult,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::observable::ResourceObservableOptions;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Destination::In + Clone + Send + Sync,
	Destination: 'static + Subscriber,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	_phantom_data: PhantomData<(R, Reader)>,
}

impl<R, Reader, Destination> ResourceSubscription<R, Reader, Destination>
where
	R: Resource,
	Reader: 'static + Fn(&R) -> Destination::In + Clone + Send + Sync,
	Destination: 'static + Subscriber,
{
	pub fn new(
		destination: Destination,
		reader: Reader,
		options: ResourceObservableOptions,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let mut shared_destination = SharedSubscriber::new(destination);

		let cancellation_id = {
			let mut scheduler_lock = scheduler.lock();
			let cancellation_id = scheduler_lock.generate_cancellation_id();

			let mut resource_existed_in_the_previous_tick = false;
			let mut shared_destination_clone = shared_destination.clone();
			scheduler_lock.schedule_continuous_work(
				move |_tick, context| {
					let is_changed = context.deferred_world.is_resource_changed::<R>();
					let resource_option = context.deferred_world.get_resource::<R>();

					let is_added = {
						let resource_exists_this_tick = resource_option.is_some();
						let is_added =
							!resource_existed_in_the_previous_tick && resource_exists_this_tick;
						resource_existed_in_the_previous_tick = resource_exists_this_tick;
						is_added
					};

					// is_changed is always true when is_added is true
					let is_changed_condition =
						options.trigger_on_is_changed && is_changed && !is_added;
					let is_added_condition = options.trigger_on_is_added && is_added;

					if (is_changed_condition || is_added_condition)
						&& let Some(resource) = resource_option
					{
						let next = (reader)(resource);
						shared_destination_clone.next(next);
					}

					if shared_destination_clone.is_closed() {
						WorkResult::Done
					} else {
						WorkResult::Pending
					}
				},
				cancellation_id,
			);

			cancellation_id
		};

		shared_destination.add(Teardown::new_work_cancellation(cancellation_id, scheduler));

		Self {
			shared_destination,
			_phantom_data: PhantomData,
		}
	}
}
