use bevy_ecs::observer::Trigger;
use bevy_time::{Timer, TimerMode};
use rx_bevy_observable::{ObservableOutput, Observer};

use rx_bevy_plugin::{
	CommandSubscriber, RxContextSub, RxDestination, RxSubscription, RxTick,
	SubscriptionChannelHandlerRegistrationContext,
};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::IntervalObservableOptions;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct IntervalSubscription {
	timer: Timer,
	count: i32,
}

impl IntervalSubscription {
	pub fn new(interval_observable_options: IntervalObservableOptions) -> Self {
		Self {
			timer: Timer::new(interval_observable_options.duration, TimerMode::Repeating),
			count: if interval_observable_options.start_on_subscribe {
				1
			} else {
				0
			},
		}
	}
}

impl ObservableOutput for IntervalSubscription {
	type Out = i32;
	type OutError = ();
}

impl RxSubscription for IntervalSubscription {
	fn register_subscription_channel_handlers<'a, 'w, 's>(
		&mut self,
		mut hooks: SubscriptionChannelHandlerRegistrationContext<'a, 'w, 's, Self>,
	) {
		hooks.register_tick_handler(interval_subscription_on_tick_system);
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}

fn interval_subscription_on_tick_system(
	trigger: Trigger<RxTick>,
	mut context: RxContextSub<IntervalSubscription>,
	mut destination: RxDestination<IntervalSubscription>,
) {
	let mut subscription = context.get_subscription(trigger.target());
	let mut subscriber = destination.get_subscriber_of(trigger.target());

	subscription.timer.tick(trigger.event().delta);
	if subscription.timer.just_finished() {
		subscriber.next(subscription.count);
		subscription.count += 1;
	}
}
