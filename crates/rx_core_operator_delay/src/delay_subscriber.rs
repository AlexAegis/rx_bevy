use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerScheduleTaskExtension, Subscriber, SubscriptionClosedFlag,
	SubscriptionLike, TaskOwnerId,
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
	owner_id: TaskOwnerId,
}

impl<Destination, S> DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(destination: Destination, mut options: DelayOperatorOptions<S>) -> Self {
		let owner_id = options.scheduler.get_scheduler().generate_owner_id();

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
			let mut scheduler = self.options.scheduler.get_scheduler();

			scheduler.schedule_delayed_task(
				move |_, _context| {
					destination.next(next);
					Ok(())
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
			let mut scheduler = self.options.scheduler.get_scheduler();
			scheduler.schedule_delayed_task(
				move |_, _context| {
					destination.complete();
					Ok(())
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
		let mut scheduler = self.options.scheduler.get_scheduler();
		let owner_id_copy = self.owner_id;

		scheduler.schedule_delayed_task(
			move |_, _context| {
				destination.unsubscribe();
				scheduler_clone.get_scheduler().cancel(owner_id_copy);
				Ok(())
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
