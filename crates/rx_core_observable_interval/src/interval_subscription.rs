use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Scheduler, SchedulerHandle, SchedulerScheduleTaskExtension, SharedSubscriber, Subscriber,
	SubscriptionData, SubscriptionLike, TaskCancellationId, TaskResult,
};

use crate::observable::IntervalObservableOptions;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct IntervalSubscription<Destination, S>
where
	Destination: Subscriber<In = usize>,
	S: Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	#[teardown]
	teardown: SubscriptionData,
	scheduler: SchedulerHandle<S>,
	task_owner_id: TaskCancellationId,
}

impl<Destination, S> IntervalSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = usize>,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		interval_subscription_options: IntervalObservableOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let mut scheduler_clone = scheduler.clone();
		let destination = SharedSubscriber::new(destination);
		let task_owner_id = {
			let mut scheduler = scheduler_clone.lock();
			let cancellation_id = scheduler.generate_cancellation_id();
			let destination_clone = destination.clone();

			let mut count = if interval_subscription_options.start_on_subscribe {
				1
			} else {
				0
			};

			scheduler.schedule_repeated_task(
				move |_, _| {
					let mut destination_lock = destination_clone.lock();

					if destination_lock.is_closed() {
						return TaskResult::Done;
					}

					destination_lock.next(count);
					count += 1;

					TaskResult::Pending
				},
				interval_subscription_options.duration,
				false,
				interval_subscription_options.max_emissions_per_tick,
				cancellation_id,
			);

			cancellation_id
		};

		IntervalSubscription {
			destination,
			teardown: SubscriptionData::default(),
			scheduler,
			task_owner_id,
		}
	}
}

impl<Destination, S> SubscriptionLike for IntervalSubscription<Destination, S>
where
	Destination: Subscriber<In = usize>,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler.lock().cancel(self.task_owner_id);
		if !self.destination.is_closed() {
			self.destination.unsubscribe();
		}
		self.teardown.unsubscribe();
	}
}
