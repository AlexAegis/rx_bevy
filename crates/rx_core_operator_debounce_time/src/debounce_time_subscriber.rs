use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	time::Duration,
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
pub struct DebounceTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	duration: Duration,
	scheduler: SchedulerHandle<S>,
	cancellation_id: WorkCancellationId,
	upstream_completed: Arc<AtomicBool>,
	upstream_unsubscribed: Arc<AtomicBool>,
	scheduled_work_counter: Arc<AtomicUsize>,
}

impl<Destination, S> DebounceTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(
		mut destination: Destination,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();
		destination.add_teardown(Teardown::new_work_cancellation(
			cancellation_id,
			scheduler.clone(),
		));
		Self {
			destination: SharedSubscriber::new(destination),
			duration,
			scheduler,
			cancellation_id,
			upstream_completed: Arc::new(AtomicBool::new(false)),
			upstream_unsubscribed: Arc::new(AtomicBool::new(false)),
			scheduled_work_counter: Arc::new(AtomicUsize::new(0)),
		}
	}
}

impl<Destination, S> RxObserver for DebounceTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		let destination = self.destination.clone();
		let scheduled_work_counter = self.scheduled_work_counter.clone();
		let upstream_completed = self.upstream_completed.clone();
		let upstream_unsubscribed = self.upstream_unsubscribed.clone();

		let mut scheduler = self.scheduler.lock();
		scheduler.cancel(self.cancellation_id);
		self.scheduled_work_counter.store(1, Ordering::Relaxed);
		scheduler.schedule_delayed_work(
			move |_, _| {
				let mut destination = destination.lock();
				if !destination.is_closed() {
					destination.next(next);
				}

				let previous_work_counter_before_sub =
					scheduled_work_counter.swap(0, Ordering::Relaxed);

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
			self.duration,
			self.cancellation_id,
		);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.upstream_unsubscribed.store(true, Ordering::Relaxed);
		self.scheduled_work_counter.store(0, Ordering::Relaxed);
		self.scheduler.lock().cancel(self.cancellation_id);
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
		}
	}
}

impl<Destination, S> SubscriptionLike for DebounceTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed() || self.upstream_unsubscribed.load(Ordering::Relaxed)
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

impl<Destination, S> Drop for DebounceTimeSubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
