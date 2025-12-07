use std::sync::{Arc, Mutex};

use bevy_time::{Timer, TimerMode};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	Never, Observer, Scheduler, SchedulerHandle, SchedulerScheduleTaskExtension, Subscriber,
	SubscriptionContext, SubscriptionData, SubscriptionLike, TaskContextItem, TaskContextProvider,
	TaskOwnerId, TeardownCollection, Tick, TickResult, Tickable,
};

use crate::observable::IntervalObservableOptions;

struct IntervalSubscriptionTaskState {
	timer: Timer,
	count: usize,
	/// It doesn't need to be a `usize` as the number it's compared against is
	/// a `u32` coming from [bevy_time::Timer::times_finished_this_tick]
	max_emissions_per_tick: u32,
}

// TODO: Remove bevy_time dependency, it's a small crate but it's versioned together with the rest of bevy, and even it could just stay on an older version for this crate, I don't want to ppl see two bevy versions in their lockfile/cargo output, that'd be confusing
#[derive(RxSubscription)]
#[rx_context(S::ContextProvider)]
pub struct IntervalSubscription<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	destination: Arc<
		Mutex<
			dyn Subscriber<In = usize, InError = Never, Context = S::ContextProvider> + Send + Sync,
		>,
	>,
	teardown: SubscriptionData<S::ContextProvider>,
	scheduler: SchedulerHandle<S>,
	task_owner_id: TaskOwnerId,
}

impl<S> IntervalSubscription<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	pub fn new(
		destination: impl Subscriber<In = usize, InError = Never, Context = S::ContextProvider>
		+ 'static,
		mut interval_subscription_options: IntervalObservableOptions<S>,
	) -> Self {
		let mut task_state = IntervalSubscriptionTaskState {
			timer: Timer::new(interval_subscription_options.duration, TimerMode::Repeating),
			count: if interval_subscription_options.start_on_subscribe {
				1
			} else {
				0
			},
			max_emissions_per_tick: interval_subscription_options.max_emissions_per_tick,
		};

		let destination = Arc::new(Mutex::new(destination));
		let task_owner_id = {
			let mut scheduler = interval_subscription_options.scheduler.get_scheduler();
			let owner_id = scheduler.generate_owner_id();
			scheduler.schedule_repeated_task(
				move |tick_input, context| {
					let delta = context.now() - task_state.timer.elapsed();
					let now = context.now();
					task_state.timer.tick(delta);

					let ticks = task_state
						.timer
						.times_finished_this_tick()
						.min(task_state.max_emissions_per_tick);
					for i in 0..ticks {
						destination.next(task_state.count + i as usize, context);
					}
					task_state.count += ticks as usize;

					Ok(())
				},
				interval_subscription_options.duration,
				false,
				owner_id,
			);

			owner_id
		};

		IntervalSubscription {
			destination,
			teardown: SubscriptionData::default(),
			scheduler: interval_subscription_options.scheduler,
			task_owner_id,
		}
	}
}

impl<S> Tickable for IntervalSubscription<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	fn tick(
		&mut self,
		_tick: Tick,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
	}
}

impl<S> SubscriptionLike for IntervalSubscription<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.scheduler.get_scheduler().cancel(self.task_owner_id);
		if !self.destination.is_closed() {
			self.destination.unsubscribe(context);
		}
		self.teardown.unsubscribe(context);
	}
}

impl<S> TeardownCollection for IntervalSubscription<S>
where
	S: Scheduler,
	S::ContextProvider: SubscriptionContext,
{
	fn add_teardown(
		&mut self,
		teardown: rx_core_traits::Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.teardown.add_teardown(teardown, context);
	}
}
