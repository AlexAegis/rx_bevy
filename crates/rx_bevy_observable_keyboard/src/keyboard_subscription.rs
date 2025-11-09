use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_context::{BevySubscriptionContext, BevySubscriptionContextProvider};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Subscriber, SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, TeardownCollection,
	Tick, Tickable,
};

#[derive(RxSubscription)]
#[rx_context(BevySubscriptionContextProvider)]
pub struct KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	destination: Destination,
	closed_flag: SubscriptionClosedFlag,
}

impl<Destination> KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed_flag: false.into(),
		}
	}
}

impl<Destination> SubscriptionLike for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[inline]
	#[track_caller]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	#[track_caller]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			self.destination.unsubscribe(context);
		}
	}
}

impl<Destination> TeardownCollection for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
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
	Destination: Subscriber<In = KeyCode, Context = BevySubscriptionContextProvider>,
{
	#[track_caller]
	fn tick(&mut self, tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		if !self.is_closed() {
			let just_pressed_key_codes = {
				let button_input = context.deferred_world.resource::<ButtonInput<KeyCode>>();
				button_input.get_just_pressed().cloned().collect::<Vec<_>>()
			};
			for key_code in just_pressed_key_codes {
				self.destination.next(key_code, context);
			}
		}

		self.destination.tick(tick, context);
	}
}

impl<Destination> Drop for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
