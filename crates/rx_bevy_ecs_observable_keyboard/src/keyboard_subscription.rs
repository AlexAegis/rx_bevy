use bevy_ecs::{event::EventReader, observer::Trigger};
use bevy_input::keyboard::KeyboardInput;
use rx_bevy_observable::{ObservableOutput, Observer, Tick};

use rx_bevy_plugin::{
	CommandSubscriber, RxContextSub, RxDestination, RxSignal, RxSubscription, RxTick,
	SubscriptionHookRegistrationContext,
};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::KeyboardObservableOptions;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct KeyboardSubscription {
	buffer: Vec<KeyboardInput>,
}

impl KeyboardSubscription {
	pub fn new(_keyboard_observable_options: KeyboardObservableOptions) -> Self {
		Self { buffer: Vec::new() }
	}

	pub(crate) fn write(&mut self, event: KeyboardInput) {
		self.buffer.push(event);
	}
}

impl ObservableOutput for KeyboardSubscription {
	type Out = KeyboardInput;
	type OutError = ();
}

impl RxSubscription for KeyboardSubscription {
	const SCHEDULED: bool = true;

	fn register_hooks<'a, 'w, 's>(
		&mut self,
		hooks: &mut SubscriptionHookRegistrationContext<'a, 'w, 's, Self>,
	) {
		hooks.register_hook(RxTick, keyboard_subscription_on_tick_system);
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}

fn keyboard_subscription_on_tick_system(
	trigger: Trigger<Tick>,
	// mut context: RxContextSub<KeyboardSubscription>,
	mut destination: RxDestination<KeyboardSubscription>,
	mut keyboard_input_events: EventReader<KeyboardInput>,
) {
	// let mut subscription = context.get_subscription(trigger.target());
	let mut subscriber = destination.get_destination(trigger.target());

	for keyboard_input in keyboard_input_events.read() {
		subscriber.next(keyboard_input.clone());
	}
}
