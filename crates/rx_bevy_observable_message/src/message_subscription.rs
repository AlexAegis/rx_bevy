use bevy_ecs::message::{Message, MessageCursor, Messages};
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::*;
use rx_core_macro_subscription_derive::RxSubscription;

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct MessageSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Message + Clone,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
}

impl<Destination> MessageSubscription<Destination>
where
	Destination: 'static + Subscriber,
	Destination::In: Message + Clone,
{
	pub fn new(destination: Destination, scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		let mut shared_destination = SharedSubscriber::new(destination);

		let cancellation_id = {
			let mut scheduler_lock = scheduler.lock();
			let cancellation_id = scheduler_lock.generate_cancellation_id();

			let mut message_cursor = MessageCursor::<Destination::In>::default();
			let shared_destination_clone = shared_destination.clone();
			scheduler_lock.schedule_continuous_work(
				move |_, context| {
					let events = context
						.deferred_world
						.resource::<Messages<Destination::In>>();

					let mut destination = shared_destination_clone.lock();
					if destination.is_closed() {
						return WorkResult::Done;
					}

					for event in message_cursor.read(events).cloned() {
						destination.next(event);

						if destination.is_closed() {
							return WorkResult::Done;
						}
					}

					WorkResult::Pending
				},
				cancellation_id,
			);
			cancellation_id
		};

		shared_destination.add(Teardown::new_work_cancellation(cancellation_id, scheduler));

		Self { shared_destination }
	}
}
