use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_context::RxBevyScheduler;
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::prelude::*;

use crate::{KeyboardObservableEmit, KeyboardObservableOptions};

#[derive(RxSubscription)]
pub struct KeyboardSubscription<Destination>
where
	Destination: Subscriber<In = KeyCode>,
{
	shared_destination: SharedSubscriber<Destination>,
	cancellation_id: TaskCancellationId,
	scheduler: SchedulerHandle<RxBevyScheduler>,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> KeyboardSubscription<Destination>
where
	Destination: 'static + Subscriber<In = KeyCode>,
{
	pub fn new(
		destination: Destination,
		options: KeyboardObservableOptions,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self {
		let shared_destination = SharedSubscriber::new(destination);

		let mut scheduler_clone = scheduler.clone();
		let mut scheduler_lock = scheduler_clone.lock();
		let cancellation_id = scheduler_lock.generate_cancellation_id();

		let mut shared_destination_clone = shared_destination.clone();
		scheduler_lock.schedule_continuous_task(
			move |_tick, context| {
				if !shared_destination_clone.is_closed() {
					let key_codes = {
						let button_input =
							context.deferred_world.resource::<ButtonInput<KeyCode>>();
						match options.emit {
							KeyboardObservableEmit::JustPressed => {
								button_input.get_just_pressed().cloned().collect::<Vec<_>>()
							}
							KeyboardObservableEmit::JustReleased => button_input
								.get_just_released()
								.cloned()
								.collect::<Vec<_>>(),
							KeyboardObservableEmit::Pressed => {
								button_input.get_pressed().cloned().collect::<Vec<_>>()
							}
						}
					};
					for key_code in key_codes {
						shared_destination_clone.next(key_code);
					}
				}
				TickResult::Pending
			},
			cancellation_id,
		);

		Self {
			shared_destination,
			cancellation_id,
			scheduler,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for KeyboardSubscription<Destination>
where
	Destination: Subscriber<In = KeyCode>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.scheduler.lock().cancel(self.cancellation_id);
			self.shared_destination.unsubscribe();
		}
	}
}

impl<Destination> TeardownCollection for KeyboardSubscription<Destination>
where
	Destination: Subscriber<In = KeyCode>,
{
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.shared_destination.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}
