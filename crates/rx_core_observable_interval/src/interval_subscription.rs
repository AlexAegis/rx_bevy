use bevy_time::{Timer, TimerMode};
use rx_core_traits::{
	Subscriber, SubscriptionContext, SubscriptionData, SubscriptionLike, Tick, Tickable,
	WithSubscriptionContext,
};

use crate::observable::IntervalObservableOptions;

// TODO: Remove bevy-time dependency, it's a small crate but it's versioned together with the rest of bevy, and even it could just stay on a version, I don't want to ppl see two bevy versions in their lockfile/cargo output, that'd be confusing
pub struct IntervalSubscription<Context>
where
	Context: SubscriptionContext,
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
	Context: SubscriptionContext,
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

impl<Context> WithSubscriptionContext for IntervalSubscription<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> Tickable for IntervalSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
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
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.destination.unsubscribe(context);
		self.teardown.unsubscribe(context);
	}

	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}

impl<Context> Drop for IntervalSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
