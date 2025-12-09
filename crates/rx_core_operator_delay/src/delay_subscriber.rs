use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerScheduleTaskExtension, Subscriber, SubscriptionClosedFlag,
	SubscriptionLike, TaskCancellationId,
};

use crate::operator::DelayOperatorOptions;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_delegate_teardown_collection_to_destination]
pub struct DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	#[destination]
	destination: Arc<Mutex<Destination>>,
	options: DelayOperatorOptions<S>,
	closed: SubscriptionClosedFlag,
	owner_id: TaskCancellationId,
}

impl<Destination, S> DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(destination: Destination, mut options: DelayOperatorOptions<S>) -> Self {
		let owner_id = options.scheduler.lock().generate_cancellation_id();

		Self {
			closed: SubscriptionClosedFlag::default(),
			destination: Arc::new(Mutex::new(destination)),
			options,
			owner_id,
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
			let mut destination = self.destination.clone();
			let mut scheduler = self.options.scheduler.lock();

			scheduler.schedule_delayed_task(
				move |_, _context| {
					destination.next(next);
				},
				self.options.delay,
				self.owner_id,
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
			let mut destination = self.destination.clone();
			let mut scheduler = self.options.scheduler.lock();
			scheduler.schedule_delayed_task(
				move |_, _context| {
					destination.complete();
				},
				self.options.delay,
				self.owner_id,
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
		let mut scheduler_clone = self.options.scheduler.clone();
		let mut scheduler = self.options.scheduler.lock();
		let owner_id_copy = self.owner_id;

		scheduler.schedule_delayed_task(
			move |_, _context| {
				destination.unsubscribe();
				scheduler_clone.lock().cancel(owner_id_copy);
			},
			self.options.delay,
			self.owner_id,
		);

		self.closed.close();
	}
}

impl<Destination, S> Drop for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	fn drop(&mut self) {
		self.closed.close();
	}
}
