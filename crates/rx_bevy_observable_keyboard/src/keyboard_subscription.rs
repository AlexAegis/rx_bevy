use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_common::RxBevyScheduler;
use rx_core_common::*;
use rx_core_macro_subscription_derive::RxSubscription;

use crate::{KeyboardObservableEmit, KeyboardObservableOptions};

#[derive(RxSubscription)]
#[rx_delegate_teardown_collection]
#[rx_delegate_subscription_like_to_destination]
pub struct KeyboardSubscription<Destination>
where
	Destination: 'static + Subscriber<In = KeyCode>,
{
	#[destination]
	shared_destination: SharedSubscriber<Destination>,
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
		let mut shared_destination = SharedSubscriber::new(destination);

		let cancellation_id = {
			let mut scheduler_lock = scheduler.lock();
			let cancellation_id = scheduler_lock.generate_cancellation_id();

			let mut shared_destination_clone = shared_destination.clone();
			scheduler_lock.schedule_continuous_work(
				move |_tick, context| {
					let button_input = context.deferred_world.resource::<ButtonInput<KeyCode>>();
					let key_code_iterator: &mut dyn Iterator<Item = KeyCode> = match options.emit {
						KeyboardObservableEmit::JustPressed => {
							&mut button_input.get_just_pressed().copied()
						}
						KeyboardObservableEmit::JustReleased => {
							&mut button_input.get_just_released().copied()
						}
						KeyboardObservableEmit::WhilePressed => {
							&mut button_input.get_pressed().copied()
						}
					};

					for key_code in key_code_iterator {
						if !shared_destination_clone.is_closed() {
							shared_destination_clone.next(key_code);
						} else {
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
