use std::time::Duration;

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerHandle, SchedulerScheduleWorkExtension, SharedSubscriber,
	Subscriber, SubscriptionClosedFlag, SubscriptionLike, WorkCancellationId,
};

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection]
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
}

impl<Destination, S> DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		duration: Duration,
		mut scheduler: SchedulerHandle<S>,
	) -> Self {
		let cancellation_id = scheduler.lock().generate_cancellation_id();

		Self {
			closed: SubscriptionClosedFlag::default(),
			destination: SharedSubscriber::new(destination),
			duration,
			scheduler,
			cancellation_id,
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
			let mut scheduler = self.scheduler.lock();

			scheduler.schedule_delayed_work(
				move |_, _| {
					let mut destination = destination.lock();
					if !destination.is_closed() {
						destination.next(next);
					}
				},
				self.duration,
				self.cancellation_id,
			);
		}
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.destination.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		if !self.is_closed() {
			let destination = self.destination.clone();
			let mut scheduler = self.scheduler.lock();
			scheduler.schedule_delayed_work(
				move |_, _context| {
					let mut destination = destination.lock();
					if !destination.is_closed() {
						destination.complete();
					}
				},
				self.duration,
				self.cancellation_id,
			);
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
		let mut destination = self.destination.clone();
		let mut scheduler_clone = self.scheduler.clone();
		let mut scheduler = self.scheduler.lock();
		let owner_id_copy = self.cancellation_id;

		scheduler.schedule_delayed_work(
			move |_, _context| {
				destination.unsubscribe();
				scheduler_clone.lock().cancel(owner_id_copy);
			},
			self.duration,
			self.cancellation_id,
		);

		self.closed.close();
	}
}
