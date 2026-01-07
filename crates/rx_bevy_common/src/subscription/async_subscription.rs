use rx_core_common::{
	Scheduler, SchedulerHandle, SubscriptionData, SubscriptionLike, WorkCancellationId,
	WorkInvokeId,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::RxBevyScheduler;

// TODO: Not really bevy specific anymore.
#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct AsyncSubscription {
	scheduler: SchedulerHandle<RxBevyScheduler>,
	#[teardown]
	teardown: SubscriptionData,
	despawn_work_id: WorkInvokeId,
	cancellation_id: WorkCancellationId,
}

impl AsyncSubscription {
	pub fn new(
		scheduler: SchedulerHandle<RxBevyScheduler>,
		cancellation_id: WorkCancellationId,
		despawn_invoke_id: WorkInvokeId,
	) -> Self {
		Self {
			despawn_work_id: despawn_invoke_id,
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
			scheduler.invoke(self.despawn_work_id);
			scheduler.cancel(self.cancellation_id);
		}
	}
}
