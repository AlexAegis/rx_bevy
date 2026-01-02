use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicUsize, Ordering},
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

fn mute_panic<R>(fun: impl FnOnce() -> R) -> R {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(|_| {}));
	let result = fun();
	std::panic::set_hook(hook);
	result
}

mod ticking {

	use super::*;

	#[test]
	fn should_be_able_to_tick_to_arbitrary_points_after_now_even_if_the_delta_is_wrong() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());
		assert_eq!(
			ticking_executor.now(),
			Duration::ZERO,
			"Initial now is not zero!"
		);

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		assert_eq!(context.now(), Duration::ZERO);

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

mod immediate_work {
	use super::*;

	#[test]
	fn should_execute_nested_immediate_work_in_a_single_tick() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let inner_work_has_executed = Arc::new(AtomicBool::default());
		let inner_work_has_executed_work = inner_work_has_executed.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let scheduler_work = ticking_executor.get_scheduler_handle();
		let scheduler_work_work = ticking_executor.get_scheduler_handle();
		{
			let mut scheduler = scheduler.lock();

			let cancellation_id_1 = scheduler.generate_cancellation_id();
			let cancellation_id_2 = scheduler.generate_cancellation_id();
			let cancellation_id_3 = scheduler.generate_cancellation_id();
			scheduler.schedule_immediate_work(
				move |_, _| {
					scheduler_work.lock().schedule_immediate_work(
						move |_, _| {
							scheduler_work_work.lock().schedule_immediate_work(
								move |_, _| {
									inner_work_has_executed_work.store(true, Ordering::Relaxed);
								},
								cancellation_id_3,
							);
						},
						cancellation_id_2,
					);
				},
				cancellation_id_1,
			);
		};

		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		ticking_executor.tick(Duration::from_millis(300), &mut context);
	}

	#[test]
	#[should_panic]
	fn should_panic_when_a_work_recursively_schedules_too_much_work() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default())
		.with_max_single_tick_recursion_depth(2);

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let inner_work_has_executed = Arc::new(AtomicBool::default());
		let inner_work_has_executed_work = inner_work_has_executed.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let scheduler_work = ticking_executor.get_scheduler_handle();
		let scheduler_work_work = ticking_executor.get_scheduler_handle();
		{
			let mut scheduler = scheduler.lock();

			let cancellation_id_1 = scheduler.generate_cancellation_id();
			let cancellation_id_2 = scheduler.generate_cancellation_id();
			let cancellation_id_3 = scheduler.generate_cancellation_id();
			scheduler.schedule_immediate_work(
				move |_, _| {
					scheduler_work.lock().schedule_immediate_work(
						move |_, _| {
							scheduler_work_work.lock().schedule_immediate_work(
								move |_, _| {
									inner_work_has_executed_work.store(true, Ordering::Relaxed);
								},
								cancellation_id_3,
							);
						},
						cancellation_id_2,
					);
				},
				cancellation_id_1,
			);
		};

		assert!(!inner_work_has_executed.load(Ordering::Relaxed));

		mute_panic(|| ticking_executor.tick(Duration::from_millis(300), &mut context));
	}
}

mod delayed_work {
	use super::*;

	#[test]
	fn should_execute_nested_delayed_work_in_individual_as_timer_always_start_from_now() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let inner_work_has_executed = Arc::new(AtomicBool::default());
		let inner_work_has_executed_work = inner_work_has_executed.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let scheduler_work = ticking_executor.get_scheduler_handle();
		let scheduler_work_work = ticking_executor.get_scheduler_handle();
		{
			let mut scheduler = scheduler.lock();

			let cancellation_id_1 = scheduler.generate_cancellation_id();
			let cancellation_id_2 = scheduler.generate_cancellation_id();
			let cancellation_id_3 = scheduler.generate_cancellation_id();
			scheduler.schedule_delayed_work(
				move |_, _| {
					scheduler_work.lock().schedule_delayed_work(
						move |_, _| {
							scheduler_work_work.lock().schedule_delayed_work(
								move |_, _| {
									inner_work_has_executed_work.store(true, Ordering::Relaxed);
								},
								Duration::from_millis(10),
								cancellation_id_3,
							);
						},
						Duration::from_millis(10),
						cancellation_id_2,
					);
				},
				Duration::from_millis(10),
				cancellation_id_1,
			);
		};

		assert!(!inner_work_has_executed.load(Ordering::Relaxed));
		ticking_executor.tick(Duration::from_millis(300), &mut context);
		assert!(!inner_work_has_executed.load(Ordering::Relaxed));
		ticking_executor.tick(Duration::from_millis(300), &mut context);
		assert!(!inner_work_has_executed.load(Ordering::Relaxed));
		ticking_executor.tick(Duration::from_millis(300), &mut context);
		assert!(inner_work_has_executed.load(Ordering::Relaxed));
	}
}

mod repeated_work {
	use super::*;

	#[test]
	fn should_be_able_to_schedule_repeated_work_and_execute_it_when_it_rolls_over() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let interval_counter = Arc::new(AtomicUsize::default());
		let interval_counter_work = interval_counter.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let cancellation_id = {
			let mut scheduler = scheduler.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_repeated_work(
				move |_, _| {
					interval_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				Duration::from_millis(1000),
				false,
				5,
				cancellation_id,
			);

			cancellation_id
		};

		assert_eq!(interval_counter.load(Ordering::Relaxed), 0);

		ticking_executor.tick(Duration::from_millis(1500), &mut context);

		assert_eq!(interval_counter.load(Ordering::Relaxed), 1);

		ticking_executor.tick(Duration::from_millis(1500), &mut context);

		assert_eq!(interval_counter.load(Ordering::Relaxed), 3);

		ticking_executor.tick(Duration::from_millis(7000), &mut context);

		assert_eq!(interval_counter.load(Ordering::Relaxed), 8); // Would be 10, but max work per tick is limited

		scheduler.lock().cancel(cancellation_id);

		ticking_executor.tick(Duration::from_millis(1000), &mut context);

		assert_eq!(
			interval_counter.load(Ordering::Relaxed),
			8,
			"Ticks after cancellation should not execute a cancelled work"
		);
	}

	#[test]
	fn should_be_able_to_start_repeated_work_immediately() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let interval_counter = Arc::new(AtomicUsize::default());
		let interval_counter_work = interval_counter.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let _cancellation_id = {
			let mut scheduler = scheduler.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_repeated_work(
				move |_, _| {
					interval_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				Duration::from_millis(1000),
				true,
				5,
				cancellation_id,
			);

			cancellation_id
		};

		assert_eq!(interval_counter.load(Ordering::Relaxed), 0);

		ticking_executor.tick(Duration::from_millis(1), &mut context);

		assert_eq!(interval_counter.load(Ordering::Relaxed), 1);

		ticking_executor.tick(Duration::from_millis(999), &mut context);

		assert_eq!(interval_counter.load(Ordering::Relaxed), 2);
	}
}

mod continuous_work {
	use super::*;

	#[test]
	fn should_be_able_to_execute_continous_work_on_every_tick_except_repeated_ticks() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());

		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let execution_counter = Arc::new(AtomicUsize::default());
		let execution_counter_work = execution_counter.clone();
		let scheduler = ticking_executor.get_scheduler_handle();
		let cancellation_id = {
			let mut scheduler = scheduler.lock();

			let cancellation_id = scheduler.generate_cancellation_id();
			scheduler.schedule_continuous_work(
				move |_, _| {
					execution_counter_work.fetch_add(1, Ordering::Relaxed);
					WorkResult::Pending
				},
				cancellation_id,
			);

			cancellation_id
		};

		assert_eq!(execution_counter.load(Ordering::Relaxed), 0);

		ticking_executor.tick(Duration::from_millis(1), &mut context);

		assert_eq!(execution_counter.load(Ordering::Relaxed), 1);

		ticking_executor.tick(Duration::from_millis(1), &mut context);

		assert_eq!(execution_counter.load(Ordering::Relaxed), 2);

		ticking_executor.tick_to(ticking_executor.get_current_tick(), &mut context);

		assert_eq!(
			execution_counter.load(Ordering::Relaxed),
			2,
			"Should not increment for double ticking!"
		);

		ticking_executor.tick(Duration::from_millis(1), &mut context);

		assert_eq!(execution_counter.load(Ordering::Relaxed), 3);

		scheduler.lock().cancel(cancellation_id);

		ticking_executor.tick(Duration::from_millis(1), &mut context);

		assert_eq!(
			execution_counter.load(Ordering::Relaxed),
			3,
			"Should not increment after cancelling!"
		);
	}
}

mod invoked_work {
	use super::*;

	#[test]
	fn should_be_able_to_execute_invoked_work() {
		let mut ticking_executor = TickingSchedulerExecutor::<
			TickingScheduler<TestContextProvider>,
			TestContextProvider,
		>::new(TickingScheduler::<TestContextProvider>::default());
		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let scheduler = ticking_executor.get_scheduler_handle();

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
		>::new(TickingScheduler::<TestContextProvider>::default());
		let mut context = TestContext {
			now: Duration::ZERO,
		};

		let scheduler = ticking_executor.get_scheduler_handle();

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
