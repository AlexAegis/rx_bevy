use std::time::Duration;

use rx_core_scheduler_ticking::{TickingScheduler, TickingSchedulerExecutor};
use rx_core_traits::{SchedulerHandle, TaskExecutor, Tick, WithTaskInputOutput};

pub struct MockExecutor {
	ticking_executor: TickingSchedulerExecutor<TickingScheduler<(), ()>, (), ()>,
}

impl Default for MockExecutor {
	fn default() -> Self {
		Self {
			ticking_executor: TickingSchedulerExecutor::new(TickingScheduler::<(), ()>::default()),
		}
	}
}

impl MockExecutor {
	pub fn tick(&mut self, tick: Tick) {
		self.ticking_executor.tick(tick, &mut ());
	}

	pub fn get_current_tick(&mut self) -> Tick {
		self.ticking_executor.get_current_tick()
	}

	pub fn tick_by_delta(&mut self, delta: Duration) {
		self.ticking_executor.tick_by_delta(delta, &mut ());
	}
}

impl WithTaskInputOutput for MockExecutor {
	type TickInput = Tick;
	type TaskError = ();
	type ContextProvider = ();
}

impl TaskExecutor for MockExecutor {
	type Scheduler = TickingScheduler<(), ()>;

	fn get_scheduler(&self) -> SchedulerHandle<Self::Scheduler> {
		self.ticking_executor.get_scheduler()
	}
}
