use bevy_time::{Timer, TimerMode};
use rx_bevy_core::{
	SignalContext, Subscriber, SubscriptionData, SubscriptionLike, Tick, Tickable, WithContext,
};

use crate::IntervalObservableOptions;

// TODO: Ensure that if a tick loops the timer over multiple times, all of them are counted and emitted
pub struct IntervalSubscription<Context>
where
	Context: SignalContext,
{
	timer: Timer,
	count: usize,
	/// It doesn't need to be a `usize` as the number it's compared against is
	/// a `u32` coming from [bevy_time::Timer::times_finished_this_tick]
	max_emissions_per_tick: u32,
	destination: Box<dyn Subscriber<In = usize, InError = (), Context = Context> + Send + Sync>,
	teardown: SubscriptionData<Context>,
}

impl<Context> IntervalSubscription<Context>
where
	Context: SignalContext,
{
	pub fn new(
		destination: impl Subscriber<In = usize, InError = (), Context = Context> + 'static,
		interval_subscription_options: IntervalObservableOptions,
	) -> Self {
		IntervalSubscription {
			timer: Timer::new(interval_subscription_options.duration, TimerMode::Repeating),
			count: if interval_subscription_options.start_on_subscribe {
				1
			} else {
				0
			},
			max_emissions_per_tick: interval_subscription_options.max_emissions_per_tick,
			destination: Box::new(destination),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<Context> WithContext for IntervalSubscription<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> Tickable for IntervalSubscription<Context>
where
	Context: SignalContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.timer.tick(tick.delta);
		let ticks = self
			.timer
			.times_finished_this_tick()
			.min(self.max_emissions_per_tick);
		for i in 0..ticks {
			self.destination.next(self.count + i as usize, context);
		}
		self.count += ticks as usize;
	}
}

impl<Context> SubscriptionLike for IntervalSubscription<Context>
where
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.destination.unsubscribe(context);
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: rx_bevy_core::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		self.teardown.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.destination.get_context_to_unsubscribe_on_drop()
	}
}

impl<Context> Drop for IntervalSubscription<Context>
where
	Context: SignalContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
