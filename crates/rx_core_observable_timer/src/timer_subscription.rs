use std::time::Duration;

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Scheduler, SchedulerHandle, SchedulerScheduleTaskExtension, SharedSubscriber, Subscriber,
	SubscriptionLike, TaskCancellationId, Teardown, TeardownCollection,
};

#[derive(RxSubscription)]
pub struct TimerSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = ()>,
	S: Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<S>,
	task_owner_id: TaskCancellationId,
}

impl<Destination, S> TimerSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = ()>,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let mut scheduler_clone = scheduler.clone();
		let destination = SharedSubscriber::new(destination);
		let task_owner_id = {
			let mut scheduler = scheduler_clone.lock();
			let cancellation_id = scheduler.generate_cancellation_id();
			let destination_clone = destination.clone();

			scheduler.schedule_delayed_task(
				move |_, _| {
					let mut destination = destination_clone.lock();
					destination.next(());
					destination.complete();
					destination.unsubscribe();
				},
				duration,
				cancellation_id,
			);

			cancellation_id
		};

		TimerSubscription {
			destination,
			scheduler,
			task_owner_id,
		}
	}
}

impl<Destination, S> SubscriptionLike for TimerSubscription<Destination, S>
where
	Destination: 'static + Subscriber<In = ()>,
	S: Scheduler,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler.lock().cancel(self.task_owner_id);
		if !self.destination.is_closed() {
			self.destination.unsubscribe();
		}
	}
}

impl<Destination, S> TeardownCollection for TimerSubscription<Destination, S>
where
	Destination: Subscriber<In = ()>,
	S: Scheduler,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		self.destination.add_teardown(teardown);
	}
}
