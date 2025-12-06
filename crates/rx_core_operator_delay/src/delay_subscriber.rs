use std::sync::{Arc, Mutex};

use rx_core_macro_subscriber_derive::RxSubscriber;
use rx_core_traits::{
	Observer, Scheduler, SchedulerScheduleTaskExtension, Subscriber, SubscriptionContext,
	SubscriptionLike, Tickable,
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
}

impl<Destination, S> DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: Scheduler,
{
	pub fn new(
		destination: Destination,
		options: DelayOperatorOptions<S>,
		_context: &mut <Destination::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
		Self {
			destination: Arc::new(Mutex::new(destination)),
			options,
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
		let mut weak_destination = self.destination.clone();
		let mut scheduler = self.options.scheduler.get_scheduler();

		// TODO: Instead of task id's use owner id's issued by schedulers, the tasks are going to be inside a slab, id'd, and a separate hashmap will store the owner_id/task_id map
		let _task_id = scheduler.schedule_delayed_task(
			move |context| {
				weak_destination.next(next, context);
				Ok(())
			},
			self.options.delay,
		);
		// self.buffer.push(Delayed {
		// 	remaining_time: self.options.delay,
		// 	item: Some(next),
		// });
		// TODO: With the better scheduler, it will be a task in the task pool. Try it in bevy
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
		// let mut weak_destination = self.destination.downgrade(context);
		let mut destination = self.destination.clone();
		let mut scheduler = self.options.scheduler.get_scheduler();
		let _task_id = scheduler.schedule_delayed_task(
			move |context| {
				destination.complete(context);
				Ok(())
			},
			self.options.delay,
		);
	}
}

impl<Destination, S> SubscriptionLike for DelaySubscriber<Destination, S>
where
	Destination: 'static + Subscriber,
	S: 'static + Scheduler<ContextProvider = Destination::Context> + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.destination.is_closed()
	}

	fn unsubscribe(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		let mut destination = self.destination.clone();
		let mut scheduler = self.options.scheduler.get_scheduler();
		let _task_id = scheduler.schedule_delayed_task(
			move |context| {
				destination.unsubscribe(context);
				Ok(())
			},
			self.options.delay,
		);
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
