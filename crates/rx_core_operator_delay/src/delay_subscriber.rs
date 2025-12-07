use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerScheduleTaskExtension, Subscriber, SubscriptionClosedFlag,
	SubscriptionContext, SubscriptionLike, TaskOwnerId, Tickable,
};

use crate::operator::DelayOperatorOptions;

#[derive(RxSubscriber)]
#[rx_in(Destination::In)]
#[rx_in_error(Destination::InError)]
#[rx_context(Destination::Context)]
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
	pub fn new(
		destination: Destination,
		mut options: DelayOperatorOptions<S>,
		_context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
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
	S: 'static + Scheduler<ContextProvider = Destination::Context> + Send + Sync,
{
	#[inline]
	fn next(
		&mut self,
		next: Self::In,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			let mut destination = self.destination.clone();
			let mut scheduler = self.options.scheduler.get_scheduler();

			scheduler.schedule_delayed_task(
				move |context| {
					destination.next(next, context);
					Ok(())
				},
				self.options.delay,
				self.owner_id,
			);
		}
	}

	#[inline]
	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.error(error, context);
	}

	#[inline]
	fn complete(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			let mut destination = self.destination.clone();
			let mut scheduler = self.options.scheduler.get_scheduler();
			scheduler.schedule_delayed_task(
				move |context| {
					destination.complete(context);
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
	S: 'static + Scheduler<ContextProvider = Destination::Context> + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed || self.destination.is_closed()
	}

	fn unsubscribe(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		let mut destination = self.destination.clone();
		let mut scheduler_clone = self.options.scheduler.clone();
		let mut scheduler = self.options.scheduler.get_scheduler();
		let owner_id_copy = self.owner_id;

		scheduler.schedule_delayed_task(
			move |context| {
				destination.unsubscribe(context);
				scheduler_clone.get_scheduler().cancel(owner_id_copy);
				Ok(())
			},
			self.options.delay,
			self.owner_id,
		);

		self.closed.close();
	}
}

impl<Destination, S> Tickable for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	fn tick(
		&mut self,
		tick: rx_core_traits::Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// SHORTCOMINGS OF THE CURRENT SETUP: the nexted item has no idea of the scheduler and doesn't know when now is, could be stored from the tick but it'd be outdated info
		// let mut nexts = Vec::<Destination::In>::new();
		// self.buffer.retain_mut(|item| {
		// 	item.remaining_time = item.remaining_time.saturating_sub(tick.delta);
		//
		// 	if item.remaining_time.is_zero() {
		// 		nexts.push(item.item.take().unwrap());
		// 		false
		// 	} else {
		// 		true
		// 	}
		// });
		// for next in nexts {
		// 	self.destination.next(next, context);
		// }

		self.destination.tick(tick, context);
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
