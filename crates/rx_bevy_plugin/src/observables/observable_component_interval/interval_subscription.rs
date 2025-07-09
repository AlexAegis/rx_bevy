use bevy_time::{Timer, TimerMode};
use rx_bevy_observable::ObservableOutput;

use crate::{
	IntervalObservableOptions, RxNext, RxTick, ScheduledSubscription, SubscriptionContext,
};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

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

impl ScheduledSubscription for IntervalSubscription {
	fn on_tick(&mut self, event: &RxTick, context: SubscriptionContext) {
		self.timer.tick(event.delta);
		if self.timer.just_finished() {
			context
				.commands
				.trigger_targets(RxNext(self.count), context.subscriber_entity);
			self.count += 1;
		}
	}

	fn unsubscribe(&mut self, _context: SubscriptionContext) {
		println!(
			"Interval unsubscribed! {}, {}",
			self.timer.elapsed_secs(),
			self.count
		);
	}
}
