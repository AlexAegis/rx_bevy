use std::{ops::Deref, time::Duration};

use rx_core_common::{WorkContext, WorkContextProvider};
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_scheduler_ticking::{Tick, TickingScheduler, TickingSchedulerExecutor};

pub struct MockContextProvider;

impl WorkContextProvider for MockContextProvider {
	type Item<'c> = MockContext;
}

pub struct MockContext;

impl WorkContext<'_> for MockContext {}

#[derive(RxExecutor)]
#[rx_context(MockContextProvider)]
#[rx_scheduler(TickingScheduler<MockContextProvider>)]
#[rx_tick(Tick)]
pub struct MockExecutor {
	#[scheduler_handle]
	ticking_executor:
		TickingSchedulerExecutor<TickingScheduler<MockContextProvider>, MockContextProvider>,
	logging_enabled: bool,
}

impl Deref for MockExecutor {
	type Target =
		TickingSchedulerExecutor<TickingScheduler<MockContextProvider>, MockContextProvider>;

	fn deref(&self) -> &Self::Target {
		&self.ticking_executor
	}
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
	pub fn tick_to(&mut self, tick: Tick) {
		if self.logging_enabled {
			println!("Ticking... ({:?})", tick);
		}
		let mut context = MockContext;
		self.ticking_executor.tick_to(tick, &mut context);
	}

	pub fn tick(&mut self, delta: Duration) {
		if self.logging_enabled {
			println!("Ticking... ({:?})", delta);
		}
		let mut context = MockContext;
		self.ticking_executor.tick(delta, &mut context);
	}
}
