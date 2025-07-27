use bevy_time::{Timer, TimerMode};
use rx_bevy_observable::{ObservableOutput, Observer, SubscriptionLike, Tick};

use crate::{CommandSubscriber, IntervalObservableOptions, ScheduledSubscription};

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
	fn on_tick(
		&mut self,
		tick: Tick,
		mut subscriber: CommandSubscriber<Self::Out, Self::OutError>,
	) {
		self.timer.tick(tick.delta);
		if self.timer.just_finished() {
			println!("interval tick");
			subscriber.next(self.count);
			self.count += 1;
		}
	}

	fn unsubscribe(&mut self, mut destination: CommandSubscriber<Self::Out, Self::OutError>) {
		destination.unsubscribe();
	}
}
/*
impl SubscriptionLike for IntervalSubscription {
	fn is_closed(&self) -> bool {}

	fn unsubscribe(&mut self) {}
	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {}
}
*/
