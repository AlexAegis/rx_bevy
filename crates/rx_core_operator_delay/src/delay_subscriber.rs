use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
	},
	time::Duration,
};

use rx_core_common::{
	Observer, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionClosedFlag, SubscriptionLike, WorkCancellationId,
};
use rx_core_macro_subscriber_derive::RxSubscriber;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler,
{
	#[destination]
	destination: SharedSubscriber<Destination>,
	duration: Duration,
	scheduler: SchedulerHandle<S>,
	closed: SubscriptionClosedFlag,
	cancellation_id: WorkCancellationId,
	upstream_completed: Arc<AtomicBool>,
	upstream_unsubscribed: Arc<AtomicBool>,
	scheduled_work_counter: Arc<AtomicUsize>,
}

impl<Destination, S> DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();

		Self {
			closed: SubscriptionClosedFlag::default(),
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

impl<Destination, S> Observer for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		if !self.is_closed() {
			let destination = self.destination.clone();
			let scheduler_clone = self.scheduler.clone();
			let cancellation_id_copy = self.cancellation_id;

			let mut scheduler = self.scheduler.lock();

			self.scheduled_work_counter.fetch_add(1, Ordering::Relaxed);
			let scheduled_work_counter = self.scheduled_work_counter.clone();
			let upstream_completed = self.upstream_completed.clone();
			let upstream_unsubscribed = self.upstream_unsubscribed.clone();
			scheduler.schedule_delayed_work(
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
					{
						destination.complete();
						scheduler_clone.lock().cancel(cancellation_id_copy);
						return;
					}

					// try unsubscribe
					if upstream_unsubscribed.load(Ordering::Relaxed) {
						if !destination.is_closed() {
							destination.unsubscribe();
						}
						scheduler_clone.lock().cancel(cancellation_id_copy);
					}
				},
				self.duration,
				self.cancellation_id,
			);
		}

		if self.destination.is_closed() {
			self.unsubscribe();
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.upstream_unsubscribed.store(true, Ordering::Relaxed);

		self.destination.error(error);
		self.closed.close();
		self.scheduler.lock().cancel(self.cancellation_id);
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			// If there is a scheduled next, just let it also complete
			self.upstream_completed.store(true, Ordering::Relaxed);
			self.upstream_unsubscribed.store(true, Ordering::Relaxed);

			// if there aren't any, complete immediately
			if self.scheduled_work_counter.load(Ordering::Relaxed) == 0 {
				self.closed.close();
				self.destination.complete();
				self.scheduler.lock().cancel(self.cancellation_id);
			} else {
				let destination = self.destination.clone();
				let scheduled_work_counter = self.scheduled_work_counter.clone();
				let scheduler_clone = self.scheduler.clone();
				let upstream_completed = self.upstream_completed.clone();
				let upstream_unsubscribed = self.upstream_unsubscribed.clone();
				let cancellation_id_copy = self.cancellation_id;

				self.scheduler.lock().schedule_delayed_work(
					move |_, _| {
						let mut destination = destination.lock();

						let previous_work_counter_before_sub =
							scheduled_work_counter.fetch_sub(1, Ordering::Relaxed);

						// try complete
						if upstream_completed.load(Ordering::Relaxed)
							&& previous_work_counter_before_sub == 1
						{
							destination.complete();
							scheduler_clone.lock().cancel(cancellation_id_copy);
							return;
						}

						// try unsubscribe
						if upstream_unsubscribed.load(Ordering::Relaxed) {
							if !destination.is_closed() {
								destination.unsubscribe();
							}
							scheduler_clone.lock().cancel(cancellation_id_copy);
						}
					},
					self.duration,
					self.cancellation_id,
				);
			}
		}
	}
}

impl<Destination, S> SubscriptionLike for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed.close();
			// If there is a scheduled next, just let it also unsubscribe
			self.upstream_unsubscribed.store(true, Ordering::Relaxed);

			// if there aren't any, unsubscribe immediately
			if self.scheduled_work_counter.load(Ordering::Relaxed) == 0 {
				self.destination.unsubscribe();
				self.scheduler.lock().cancel(self.cancellation_id);
			}
		}
	}
}

impl<Destination, S> Drop for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler + Send + Sync,
{
	fn drop(&mut self) {
		self.unsubscribe();
		self.closed.close();
	}
}
