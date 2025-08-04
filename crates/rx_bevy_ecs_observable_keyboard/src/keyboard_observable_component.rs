use bevy_ecs::component::Component;
use bevy_input::keyboard::KeyboardInput;
use rx_bevy_observable::ObservableOutput;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use rx_bevy_plugin::{
	CommandSubscriber, ObservableComponent, ObservableOnInsertContext, OnInsertSubHook,
	observable_on_insert_hook, observable_on_remove_hook,
};

use crate::{KeyboardObservableOptions, KeyboardSubscription};

#[derive(Component, Clone)]
#[component(on_insert = observable_on_insert_hook::<Self>, on_remove = observable_on_remove_hook::<<Self as ObservableComponent>::Subscription>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct KeyboardObservableComponent {
	options: KeyboardObservableOptions,
}

impl KeyboardObservableComponent {
	pub fn new(options: KeyboardObservableOptions) -> Self {
		Self { options }
	}
}

impl ObservableOutput for KeyboardObservableComponent {
	type Out = KeyboardInput;
	type OutError = ();
}

impl ObservableComponent for KeyboardObservableComponent {
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = KeyboardSubscription;

	fn on_subscribe(
		&mut self,
		mut _subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		KeyboardSubscription::new(self.options.clone())
	}
}

impl OnInsertSubHook for KeyboardObservableComponent {
	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}
}
