use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_context::{BevySubscriptionContext, BevySubscriptionContextProvider};
use rx_core_traits::{
	Subscriber, SubscriptionContext, SubscriptionLike, Tick, Tickable, WithSubscriptionContext,
};

pub struct KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	destination: Destination,
	closed: bool,
}

impl<Destination> KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
		}
	}
}

impl<Destination> WithSubscriptionContext for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	type Context = BevySubscriptionContextProvider;
}

impl<Destination> SubscriptionLike for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.closed = true;
		self.destination.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.destination.add_teardown(teardown, context);
	}
}

impl<Destination> Tickable for KeyboardSubscription<Destination>
where
	Destination: Subscriber<In = KeyCode, Context = BevySubscriptionContextProvider>,
{
	fn tick(&mut self, _tick: Tick, context: &mut BevySubscriptionContext<'_, '_>) {
		let just_pressed_key_codes = {
			let button_input = context.deferred_world.resource::<ButtonInput<KeyCode>>();
			button_input.get_just_pressed().cloned().collect::<Vec<_>>()
		};

		println!("ticked!");
		for key_code in just_pressed_key_codes {
			self.destination.next(key_code, context);
		}
	}
}

impl<Destination> Drop for KeyboardSubscription<Destination>
where
	Destination: Subscriber<Context = BevySubscriptionContextProvider>,
{
	fn drop(&mut self) {
		let mut context = BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
		self.unsubscribe(&mut context);
	}
}
