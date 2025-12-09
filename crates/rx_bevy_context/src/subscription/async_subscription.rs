use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Scheduler, SchedulerHandle, SubscriptionData, SubscriptionLike, TaskCancellationId,
	TaskInvokeId, Teardown, TeardownCollection,
};

use crate::RxBevyScheduler;

// TODO: Not really bevy specific anymore.
#[derive(RxSubscription)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct AsyncSubscription {
	scheduler: SchedulerHandle<RxBevyScheduler>,
	teardown: SubscriptionData,
	despawn_task_id: TaskInvokeId,
	cancellation_id: TaskCancellationId,
}

impl AsyncSubscription {
	pub fn new(
		scheduler: SchedulerHandle<RxBevyScheduler>,
		cancellation_id: TaskCancellationId,
		despawn_invoke_id: TaskInvokeId,
	) -> Self {
		Self {
			despawn_task_id: despawn_invoke_id,
			cancellation_id,
			scheduler,
			teardown: SubscriptionData::default(),
		}
	}
}

impl SubscriptionLike for AsyncSubscription {
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.teardown.unsubscribe();
			let mut scheduler = self.scheduler.lock();
			scheduler.invoke(self.despawn_task_id);
			scheduler.cancel(self.cancellation_id);
		}
	}
}

impl TeardownCollection for AsyncSubscription {
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.teardown.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl Drop for AsyncSubscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
