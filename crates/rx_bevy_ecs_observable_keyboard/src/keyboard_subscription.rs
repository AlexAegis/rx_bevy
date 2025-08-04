use bevy_input::keyboard::KeyboardInput;
use rx_bevy_observable::{ObservableOutput, Tick};

use rx_bevy_plugin::{CommandSubscriber, ObserverSignalPush, RxSubscription};

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

// TODO: CONTINUE it would be nice to have a query here accessible, defined as an associated type maybe? and then the hooks would just populate it
impl RxSubscription for KeyboardSubscription {
	fn on_tick(
		&mut self,
		_tick: Tick,
		mut _subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		for event in self.buffer.drain(..) {
			_subscriber.push(rx_bevy_plugin::RxSignal::Next(event));
		}
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}
