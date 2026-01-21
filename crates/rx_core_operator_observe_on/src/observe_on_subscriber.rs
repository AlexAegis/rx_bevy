use std::sync::{
	Arc,
	atomic::{AtomicBool, AtomicUsize, Ordering},
};

use rx_core_common::{
	RxObserver, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionLike, Teardown, WorkCancellationId,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct ObserveOnSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<S>,
	cancellation_id: WorkCancellationId,
	upstream_completed: Arc<AtomicBool>,
	upstream_unsubscribed: Arc<AtomicBool>,
	scheduled_work_counter: Arc<AtomicUsize>,
}

impl<Destination, S> ObserveOnSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(mut destination: Destination, scheduler: SchedulerHandle<S>) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();
		destination.add_teardown(Teardown::new_work_cancellation(
			cancellation_id,
			scheduler.clone(),
		));
		Self {
			destination: SharedSubscriber::new(destination),
			scheduler,
			cancellation_id,
			upstream_completed: Arc::new(AtomicBool::new(false)),
			upstream_unsubscribed: Arc::new(AtomicBool::new(false)),
			scheduled_work_counter: Arc::new(AtomicUsize::new(0)),
		}
	}
}

impl<Destination, S> RxObserver for ObserveOnSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let destination = self.destination.clone();

		self.scheduled_work_counter.fetch_add(1, Ordering::Relaxed);
		let scheduled_work_counter = self.scheduled_work_counter.clone();
		let upstream_completed = self.upstream_completed.clone();
		let upstream_unsubscribed = self.upstream_unsubscribed.clone();
		self.scheduler.lock().schedule_immediate_work(
			move |_, _| {
				let mut destination = destination.lock();
				if !destination.is_closed() {
					destination.next(next);
				}

				let previous_work_counter_before_sub =
					scheduled_work_counter.fetch_sub(1, Ordering::Relaxed);

				// try complete
				if upstream_completed.load(Ordering::Relaxed)
					&& previous_work_counter_before_sub == 1
					&& !destination.is_closed()
				{
					destination.complete();
					return;
				}

				// try unsubscribe
				if upstream_unsubscribed.load(Ordering::Relaxed)
					&& previous_work_counter_before_sub == 1
					&& !destination.is_closed()
				{
					destination.unsubscribe();
				}
			},
			self.cancellation_id,
		);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.upstream_unsubscribed.store(true, Ordering::Relaxed);
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		// If there is a scheduled next, just let it also complete
		self.upstream_completed.store(true, Ordering::Relaxed);
		self.upstream_unsubscribed.store(true, Ordering::Relaxed);

		// if there aren't any, complete immediately
		if self.scheduled_work_counter.load(Ordering::Relaxed) == 0 {
			self.destination.complete();
		} else {
			let scheduled_work_counter = self.scheduled_work_counter.clone();

			self.scheduler.lock().schedule_immediate_work(
				move |_, _| {
					scheduled_work_counter.fetch_sub(1, Ordering::Relaxed);
				},
				self.cancellation_id,
			);
		}
	}
}

impl<Destination, S> SubscriptionLike for ObserveOnSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			// If there is a scheduled next, just let it also unsubscribe
			self.upstream_unsubscribed.store(true, Ordering::Relaxed);

			// if there aren't any, unsubscribe immediately
			if self.scheduled_work_counter.load(Ordering::Relaxed) == 0 {
				self.destination.unsubscribe();
			}
		}
	}
}

impl<Destination, S> Drop for ObserveOnSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
