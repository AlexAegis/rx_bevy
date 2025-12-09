use std::time::Duration;

use rx_core_macro_executor_derive::RxExecutor;
use rx_core_scheduler_ticking::{Tick, TickingScheduler, TickingSchedulerExecutor};
use rx_core_traits::SchedulerHandle;

#[derive(RxExecutor)]
#[rx_context(())]
#[rx_scheduler(TickingScheduler<()>)]
#[rx_tick(Tick)]
pub struct MockExecutor {
	#[scheduler_handle]
	ticking_executor: TickingSchedulerExecutor<TickingScheduler<()>, ()>,
}

impl Default for MockExecutor {
	fn default() -> Self {
		Self {
			ticking_executor: TickingSchedulerExecutor::new(TickingScheduler::<()>::default()),
		}
	}
}

impl MockExecutor {
	pub fn tick(&mut self, tick: Tick) {
		println!("Ticking... ({:?})", tick);
		self.ticking_executor.tick(tick, &mut ());
	}

	pub fn get_current_tick(&mut self) -> Tick {
		self.ticking_executor.get_current_tick()
	}

	pub fn tick_by_delta(&mut self, delta: Duration) {
		println!("Ticking... ({:?})", delta);
		self.ticking_executor.tick_by_delta(delta, &mut ());
	}
}
