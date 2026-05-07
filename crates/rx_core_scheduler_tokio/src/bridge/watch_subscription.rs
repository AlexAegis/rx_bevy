use std::sync::{Arc, Mutex};

use rx_core_common::{
	Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber, Signal,
	Subscriber, SubscriptionLike, TeardownCollectionExtension, WorkCancellationId, WorkResult,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::TokioScheduler;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct WatchSubscription<Destination, T>
where
	Destination: 'static + Subscriber<In = T>,
	T: Signal + Clone,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<TokioScheduler>,
	cancellation_id: WorkCancellationId,
}

impl<Destination, T> WatchSubscription<Destination, T>
where
	Destination: 'static + Subscriber<In = T>,
	T: Signal + Clone,
{
	pub fn new(
		destination: Destination,
		receiver: tokio::sync::watch::Receiver<T>,
		scheduler: SchedulerHandle<TokioScheduler>,
	) -> Self {
		let mut destination = SharedSubscriber::new(destination);
		let receiver = Arc::new(Mutex::new(receiver));

		let cancellation_id = {
			let mut scheduler_lock = scheduler.lock();
			let cancellation_id = scheduler_lock.generate_cancellation_id();
			let destination_clone = destination.clone();

			scheduler_lock.schedule_continuous_work(
				move |_, _| {
					let Ok(mut receiver) = receiver.lock() else {
						return WorkResult::Pending;
					};

					let mut dest = destination_clone.lock();
					if dest.is_closed() {
						return WorkResult::Done;
					}

					if receiver.has_changed().unwrap_or(false) {
						let value = receiver.borrow_and_update().clone();
						dest.next(value);
						if dest.is_closed() {
							return WorkResult::Done;
						}
					}

					WorkResult::Pending
				},
				cancellation_id,
			);

			cancellation_id
		};

		let scheduler_clone = scheduler.clone();
		destination.add_fn(move || {
			scheduler_clone.lock().cancel(cancellation_id);
		});

		Self {
			destination,
			scheduler,
			cancellation_id,
		}
	}
}

impl<Destination, T> SubscriptionLike for WatchSubscription<Destination, T>
where
	Destination: Subscriber<In = T>,
	T: Signal + Clone,
{
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		self.scheduler.lock().cancel(self.cancellation_id);
		if !self.destination.is_closed() {
			self.destination.unsubscribe();
		}
	}
}
