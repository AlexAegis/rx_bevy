use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_scheduler_ticking::{Tick, TickingScheduler, TickingSchedulerExecutor};

use rx_core_scheduler_ticking::TickedInvokedWorkFactory;

struct TestContextProvider;

impl WorkContextProvider for TestContextProvider {
	type Item<'c> = TestContext;
}

struct TestContext {
	now: Duration,
}

impl WorkContext<'_> for TestContext {
	fn now(&self) -> Duration {
		self.now
	}
}

mod ticking {
	use super::*;

	#[test]
	fn should_be_able_to_tick_to_arbitrary_points_after_now_even_if_the_delta_is_wrong() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::default();
		assert_eq!(
			ticking_executor.now(),
			Duration::ZERO,
			"Initial now is not zero!"
		);

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		ticking_executor.tick_to(
			Tick::new(Duration::from_millis(1500), Duration::from_millis(1500)),
			&mut context,
		);

		assert_eq!(
			ticking_executor.now(),
			Duration::from_millis(1500),
			"Ticked now is not correct!"
		);

		ticking_executor.tick_to(
			Tick::new(Duration::from_millis(10000), Duration::from_millis(0)),
			&mut context,
		);

		assert_eq!(
			ticking_executor.now(),
			Duration::from_millis(10000),
			"Ticked now is not correct!"
		);
	}
}

mod invokation {

	use super::*;

	#[test]
	fn should_be_able_to_execute_invoked_work() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::default();
		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let mut scheduler = ticking_executor.get_scheduler_handle();

		let was_invoked = Arc::new(AtomicBool::new(false));
		let was_invoked_clone = was_invoked.clone();
		let invoked_work = TickedInvokedWorkFactory::new(move |_, _| {
			was_invoked_clone.store(true, Ordering::Relaxed);
			WorkResult::Done
		});

		let invoke_id = {
			let mut scheduler = scheduler.lock();
			let invoke_id = scheduler.generate_invoke_id();
			scheduler.schedule_invoked_work(invoked_work, invoke_id);
			invoke_id
		};

		assert!(
			!was_invoked.load(Ordering::Relaxed),
			"Should not have been invoked before invoking work"
		);

		scheduler.lock().invoke(invoke_id);

		assert!(
			!was_invoked.load(Ordering::Relaxed),
			"Should not have been invoked before also ticking the executor"
		);

		ticking_executor.tick(Duration::from_millis(0), &mut context);

		assert!(
			was_invoked.load(Ordering::Relaxed),
			"Work should have been invoked!"
		);
	}

	#[test]
	fn should_be_able_to_cancel_invoked_work() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::default();
		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let mut scheduler = ticking_executor.get_scheduler_handle();

		let was_invoked = Arc::new(AtomicBool::new(false));
		let was_invoked_clone = was_invoked.clone();
		let invoked_work = TickedInvokedWorkFactory::new(move |_, _| {
			was_invoked_clone.store(true, Ordering::Relaxed);
			WorkResult::Done
		});

		let invoke_id = {
			let mut scheduler = scheduler.lock();
			let invoke_id = scheduler.generate_invoke_id();
			scheduler.schedule_invoked_work(invoked_work, invoke_id);
			invoke_id
		};

		assert!(
			!was_invoked.load(Ordering::Relaxed),
			"Should not have been invoked before invoking work"
		);

		{
			let mut scheduler = scheduler.lock();
			scheduler.cancel_invoked(invoke_id);
			scheduler.invoke(invoke_id);
		}

		ticking_executor.tick(Duration::from_millis(0), &mut context);

		assert!(
			!was_invoked.load(Ordering::Relaxed),
			"Should not have been invoked because it was cancelled!"
		);
	}
}
