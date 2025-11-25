use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_context::{RxBevyContext, RxBevyContextItem};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, TeardownCollection,
	Tick, Tickable,
};

use crate::{KeyboardObservableEmit, KeyboardObservableOptions};

#[derive(RxSubscription)]
#[rx_context(RxBevyContext)]
pub struct KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = RxBevyContext>,
{
	destination: Destination,
	options: KeyboardObservableOptions,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = RxBevyContext>,
{
	pub fn new(destination: Destination, options: KeyboardObservableOptions) -> Self {
		Self {
			destination,
			options,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = RxBevyContext>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = RxBevyContext>,
{
	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			self.destination.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<Destination> Tickable for KeyboardSubscription<Destination>
where
	Destination: Subscriber<In = KeyCode, Context = RxBevyContext>,
{
	fn tick(&mut self, tick: Tick, context: &mut RxBevyContextItem<'_, '_>) {
		if !self.is_closed() {
			let key_codes = {
				let button_input = context.deferred_world.resource::<ButtonInput<KeyCode>>();
				match self.options.emit {
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
				self.destination.next(key_code, context);
			}
		}

		self.destination.tick(tick, context);
	}
}

impl<Destination> Drop for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = RxBevyContext>,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = RxBevyContext::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
