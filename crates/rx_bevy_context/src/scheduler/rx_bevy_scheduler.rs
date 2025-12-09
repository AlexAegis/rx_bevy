use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_scheduler_ticking::{
	ContinuousTaskTickedFactory, DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory,
	InvokedTaskTickedFactory, RepeatedTaskTickedFactory, Tick, TickingExecutorsScheduler,
	TickingScheduler,
};
use rx_core_traits::{ScheduledTaskAction, Scheduler, Task, TaskCancellationId, TaskInvokeId};

use crate::RxBevyContext;

#[derive(Default, Debug, RxScheduler)]
#[rx_tick(Tick)]
#[rx_context(RxBevyContext)]
pub struct RxBevyScheduler {
	ticking_scheduler: TickingScheduler<RxBevyContext>,
}

impl TickingExecutorsScheduler for RxBevyScheduler {
	#[inline]
	fn drain_tasks(&mut self) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, RxBevyContext>> {
		self.ticking_scheduler.drain_tasks()
	}

	#[inline]
	fn has_tasks(&self) -> bool {
		self.ticking_scheduler.has_tasks()
	}

	#[inline]
	fn update_tick(&mut self, tick: Tick) {
		self.ticking_scheduler.update_tick(tick);
	}
}

impl Scheduler for RxBevyScheduler {
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<RxBevyContext>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<RxBevyContext>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<RxBevyContext>;
	type InvokedTaskFactory = InvokedTaskTickedFactory<RxBevyContext>;
	type ContinuousTaskFactory = ContinuousTaskTickedFactory<RxBevyContext>;

	#[inline]
	fn schedule_task<T>(&mut self, task: T, owner_id: TaskCancellationId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync,
	{
		self.ticking_scheduler.schedule_task(task, owner_id);
	}

	#[inline]
	fn schedule_invoked_task<T>(&mut self, task: T, invoke_id: TaskInvokeId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync,
	{
		self.ticking_scheduler
			.schedule_invoked_task(task, invoke_id);
	}

	#[inline]
	fn invoke(&mut self, invoke_id: TaskInvokeId) {
		self.ticking_scheduler.invoke(invoke_id);
	}

	#[inline]
	fn cancel(&mut self, owner_id: TaskCancellationId) {
		self.ticking_scheduler.cancel(owner_id);
	}

	#[inline]
	fn cancel_invoked(&mut self, invoke_id: TaskInvokeId) {
		self.ticking_scheduler.cancel_invoked(invoke_id);
	}

	#[inline]
	fn generate_cancellation_id(&mut self) -> TaskCancellationId {
		self.ticking_scheduler.generate_cancellation_id()
	}

	#[inline]
	fn generate_invoke_id(&mut self) -> TaskInvokeId {
		self.ticking_scheduler.generate_invoke_id()
	}
}
