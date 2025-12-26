use std::time::Duration;

use rx_core_macro_executor_derive::RxExecutor;
use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_scheduler_ticking::{
	ContinuousTaskTickedFactory, DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory,
	InvokedTaskTickedFactory, RepeatedTaskTickedFactory, Tick,
};
use rx_core_traits::{
	ContextProvider, Scheduler, SchedulerHandle, Task, TaskCancellationId, TaskContext,
	TaskInvokeId,
};

#[derive(RxScheduler)]
#[rx_context(FakeContext)]
#[rx_tick(Tick)]
struct FakeScheduler;

impl Scheduler for FakeScheduler {
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<FakeContext>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<FakeContext>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<FakeContext>;
	type InvokedTaskFactory = InvokedTaskTickedFactory<FakeContext>;
	type ContinuousTaskFactory = ContinuousTaskTickedFactory<FakeContext>;

	fn cancel(&mut self, _cancellation_id: TaskCancellationId) {}

	fn cancel_invoked(&mut self, _invoke_id: TaskInvokeId) {}

	fn generate_cancellation_id(&mut self) -> TaskCancellationId {
		unreachable!()
	}
	fn generate_invoke_id(&mut self) -> TaskInvokeId {
		unreachable!()
	}
	fn invoke(&mut self, _invoke_id: TaskInvokeId) {}
	fn schedule_invoked_task<T>(&mut self, _task: T, _invoke_id: TaskInvokeId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync,
	{
	}
	fn schedule_task<T>(&mut self, _task: T, _cancellation_id: TaskCancellationId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync,
	{
	}
}

struct FakeTaskContext;

impl<'a> TaskContext<'a> for FakeTaskContext {
	fn now(&self) -> Duration {
		Duration::ZERO
	}
}

struct FakeContext;

impl ContextProvider for FakeContext {
	type Item<'c> = FakeTaskContext;
}

#[derive(RxExecutor)]
#[rx_context(FakeContext)]
#[rx_tick(Tick)]
#[rx_scheduler(FakeScheduler)]
struct FakeExecutor {
	#[scheduler_handle]
	scheduler: SchedulerHandle<FakeScheduler>,
}

#[test]
fn compiles() {
	let fake_scheduler = FakeScheduler;
	let _fake_executor = FakeExecutor {
		scheduler: SchedulerHandle::new(fake_scheduler),
	};
}
