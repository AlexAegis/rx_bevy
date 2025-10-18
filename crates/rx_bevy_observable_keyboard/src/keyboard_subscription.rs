use std::marker::PhantomData;

use bevy_ecs::system::{Res, SystemParam};
use bevy_input::{ButtonInput, keyboard::KeyCode};
use rx_bevy_context::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	EntitySubscriptionContextAccessProvider,
};
use rx_core_traits::{
	Subscriber, SubscriptionLike, Tick, Tickable,
	prelude::{SubscriptionContext, WithSubscriptionContext},
};

pub struct KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
{
	destination: Destination,
	closed: bool,
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<Destination, ContextAccess> KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			destination,
			closed: false,
			_phantom_data: PhantomData,
		}
	}
}

impl<Destination, ContextAccess> WithSubscriptionContext
	for KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<Destination, ContextAccess> SubscriptionLike
	for KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
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

impl<Destination, ContextAccess> Tickable for KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<In = KeyCode, Context = BevySubscriptionContextProvider<ContextAccess>>,
{
	fn tick(&mut self, _tick: Tick, context: &mut BevySubscriptionContext<'_, '_, ContextAccess>) {
		let just_pressed_key_codes = {
			let button_input = context.deferred_world.resource::<ButtonInput<KeyCode>>();
			button_input.get_just_pressed().cloned().collect::<Vec<_>>()
		};

		for key_code in just_pressed_key_codes {
			self.destination.next(key_code, context);
		}
	}
}

impl<Destination, ContextAccess> Drop for KeyboardSubscription<Destination, ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
	Destination: Subscriber<Context = BevySubscriptionContextProvider<ContextAccess>>,
{
	fn drop(&mut self) {
		let mut context =
			BevySubscriptionContextProvider::<ContextAccess>::create_context_to_unsubscribe_on_drop(
			);
		self.unsubscribe(&mut context);
	}
}
