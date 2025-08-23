use bevy_ecs::component::Component;
use rx_bevy_observable::{ObservableOutput, Observer};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use rx_bevy_plugin::{
	CommandSubscriber, ObservableComponent, ObservableOnInsertContext, OnInsertSubHook,
	observable_on_insert_hook, observable_on_remove_hook,
};

use crate::{IntervalObservableOptions, IntervalSubscription};

#[derive(Component, Clone)]
#[component(on_insert = observable_on_insert_hook::<Self>, on_remove = observable_on_remove_hook::<Self>)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalObservableComponent {
	options: IntervalObservableOptions,
}

impl IntervalObservableComponent {
	pub fn new(options: IntervalObservableOptions) -> Self {
		Self { options }
	}
}

impl ObservableOutput for IntervalObservableComponent {
	type Out = i32;
	type OutError = ();
}

impl ObservableComponent for IntervalObservableComponent {
	const CAN_SELF_SUBSCRIBE: bool = true;

	type Subscription = IntervalSubscription;

	fn on_subscribe(
		&mut self,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) -> Self::Subscription {
		if self.options.start_on_subscribe {
			subscriber.next(0);
		}
		IntervalSubscription::new(self.options.clone())
	}
}

impl OnInsertSubHook for IntervalObservableComponent {
	fn on_insert(&mut self, _context: ObservableOnInsertContext) {}
}
