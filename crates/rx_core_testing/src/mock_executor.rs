use std::time::Duration;

use rx_core_macro_executor_derive::RxExecutor;
use rx_core_scheduler_ticking::{Tick, TickingScheduler, TickingSchedulerExecutor};
use rx_core_traits::{ContextProvider, SchedulerHandle, TaskContext};

pub struct TickingContextProvider;

impl ContextProvider for TickingContextProvider {
	type Item<'c> = TickingContext;
}

pub struct TickingContext {
	now: Duration,
}

impl TickingContext {
	fn new(now: Duration) -> Self {
		Self { now }
	}
}

impl TaskContext<'_> for TickingContext {
	fn now(&self) -> Duration {
		self.now
	}
}

#[derive(RxExecutor)]
#[rx_context(TickingContextProvider)]
#[rx_scheduler(TickingScheduler<TickingContextProvider>)]
#[rx_tick(Tick)]
pub struct MockExecutor {
	#[scheduler_handle]
	ticking_executor:
		TickingSchedulerExecutor<TickingScheduler<TickingContextProvider>, TickingContextProvider>,
	logging_enabled: bool,
}

impl MockExecutor {
	pub fn new_with_logging() -> Self {
		Self {
			logging_enabled: true,
			ticking_executor: TickingSchedulerExecutor::new(TickingScheduler::default()),
		}
	}
}

impl Default for MockExecutor {
	fn default() -> Self {
		Self {
			logging_enabled: false,
			ticking_executor: TickingSchedulerExecutor::new(TickingScheduler::default()),
		}
	}
}

impl MockExecutor {
	pub fn tick(&mut self, tick: Tick) {
		if self.logging_enabled {
			println!("Ticking... ({:?})", tick);
		}
		let mut context = TickingContext::new(self.ticking_executor.now());
		self.ticking_executor.tick(tick, &mut context);
	}

	pub fn get_current_tick(&mut self) -> Tick {
		self.ticking_executor.get_current_tick()
	}

	pub fn tick_by_delta(&mut self, delta: Duration) {
		if self.logging_enabled {
			println!("Ticking... ({:?})", delta);
		}
		let mut context = TickingContext::new(self.ticking_executor.now());
		self.ticking_executor.tick_by_delta(delta, &mut context);
	}
}
