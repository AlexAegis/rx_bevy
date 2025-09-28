use bevy_ecs::{event::EventReader, observer::Trigger};
use bevy_input::keyboard::KeyboardInput;
use rx_bevy_core::ObservableOutput;

use rx_bevy_plugin::{
	CommandSubscriber, RxSubscription, RxTick, SubscriptionChannelHandlerRegistrationContext,
};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::KeyboardObservableOptions;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct KeyboardSubscription;

impl KeyboardSubscription {
	pub fn new(_keyboard_observable_options: KeyboardObservableOptions) -> Self {
		Self
	}
}

impl ObservableOutput for KeyboardSubscription {
	type Out = KeyboardInput;
	type OutError = ();
}

impl RxSubscription for KeyboardSubscription {
	const SCHEDULED: bool = true;

	fn register_subscription_channel_handlers<'a, 'w, 's>(
		&mut self,
		mut _hooks: SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		// hooks.register_tick_handler(keyboard_subscription_on_tick_system);
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}

fn _keyboard_subscription_on_tick_system(
	_trigger: Trigger<RxTick>,
	// mut destination: RxDestination<KeyboardSubscription>,
	mut _keyboard_input_events: EventReader<KeyboardInput>,
) {
	// let mut subscriber = destination.get_subscriber_of(trigger.target());

	// for keyboard_input in keyboard_input_events.read() {
	// 	subscriber.next(keyboard_input.clone());
	// }
	//
	// subscriber.tick(trigger.0.clone());
}
