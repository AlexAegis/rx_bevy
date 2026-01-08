use bevy_ecs::event::{Event, EventCursor, Events};
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::*;
use rx_core_macro_subscription_derive::RxSubscription;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
pub struct MessageSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone, // TODO(bevy-0.17): use the message trait
{
	#[teardown]
	teardown: SubscriptionData,
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	cancellation_id: WorkCancellationId,
}

impl<Destination> MessageSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	pub fn new(destination: Destination, scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		let shared_destination = SharedSubscriber::new(destination);

		let scheduler_clone = scheduler.clone();
		let mut scheduler_lock = scheduler_clone.lock();
		let cancellation_id = scheduler_lock.generate_cancellation_id();
		let shared_destination_clone = shared_destination.clone();

		let mut message_cursor = EventCursor::<Destination::In>::default();

		scheduler_lock.schedule_continuous_work(
			move |_, context| {
				let events = context.deferred_world.resource::<Events<Destination::In>>();

				let read_events = message_cursor.read(events).cloned().collect::<Vec<_>>();

				let mut destination = shared_destination_clone.lock();
				if destination.is_closed() {
					return WorkResult::Done;
				}

				for event in read_events {
					destination.next(event);
				}

				WorkResult::Pending
			},
			cancellation_id,
		);
		Self {
			shared_destination,
			scheduler,
			cancellation_id,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Destination> SubscriptionLike for MessageSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Event + Clone,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.scheduler.lock().cancel(self.cancellation_id);
			self.shared_destination.unsubscribe();
			self.teardown.unsubscribe();
		}
	}
}
