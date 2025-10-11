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
	count: u32,
	max_emissions_per_tick: u32,
	destination: Box<dyn Subscriber<In = u32, InError = (), Context = Context> + Send + Sync>,
	teardown: SubscriptionData<Context>,
}

impl<Context> IntervalSubscription<Context>
where
	Context: SignalContext,
{
	pub fn new(
		destination: impl Subscriber<In = u32, InError = (), Context = Context> + Send + Sync + 'static,
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
			self.destination.next(self.count + i, context);
		}
		self.count += ticks;
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
