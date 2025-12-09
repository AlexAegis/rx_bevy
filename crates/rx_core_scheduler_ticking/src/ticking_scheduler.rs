use derive_where::derive_where;
use rx_core_macro_scheduler_derive::RxScheduler;
use rx_core_traits::{
	ContextProvider, ScheduledTaskAction, Scheduler, Task, TaskCancellationId,
	TaskCancellationIdGenerator, TaskInvokeId, TaskInvokeIdGenerator,
};

use crate::{
	ContinuousTaskTickedFactory, DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory,
	InvokedTaskTickedFactory, RepeatedTaskTickedFactory, Tick, TickingExecutorsScheduler,
};

#[derive(RxScheduler)]
#[derive_where(Default, Debug)]
#[rx_tick(Tick)]
#[rx_context(C)]
pub struct TickingScheduler<C = ()>
where
	C: ContextProvider,
{
	task_cancellation_id_generator: TaskCancellationIdGenerator,
	task_invoke_id_generator: TaskInvokeIdGenerator,
	/// Updated by the executor at the start of each tick.
	pub(crate) current_tick: Tick,
	task_action_queue: Vec<ScheduledTaskAction<Tick, C>>,
}

impl<C> TickingExecutorsScheduler for TickingScheduler<C>
where
	C: 'static + ContextProvider + Send + Sync,
{
	fn drain_tasks(&mut self) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, C>> {
		self.task_action_queue.drain(..)
	}

	fn has_tasks(&self) -> bool {
		self.task_action_queue.len() > 0
	}

	fn update_tick(&mut self, tick: Tick) {
		self.current_tick.update(tick);
	}
}

impl<C> Scheduler for TickingScheduler<C>
where
	C: 'static + ContextProvider + Send + Sync,
{
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<C>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<C>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<C>;
	type InvokedTaskFactory = InvokedTaskTickedFactory<C>;
	type ContinuousTaskFactory = ContinuousTaskTickedFactory<C>;

	fn schedule_task<T>(&mut self, mut task: T, owner_id: TaskCancellationId)
	where
		T: 'static + Task<Tick = Tick, ContextProvider = C> + Send + Sync,
	{
		task.on_scheduled_hook(self.current_tick);

		self.task_action_queue
			.push(ScheduledTaskAction::Activate((owner_id, Box::new(task))));

		//	let mut s = SubscriptionData::<ContextProvider>::default();
		// TODO: Try returning subscriptions instead of ownerids
	}

	fn schedule_invoked_task<T>(&mut self, mut task: T, invoke_id: TaskInvokeId)
	where
		T: 'static + Task<Tick = Self::Tick, ContextProvider = Self::ContextProvider> + Send + Sync,
	{
		task.on_scheduled_hook(self.current_tick);

		self.task_action_queue
			.push(ScheduledTaskAction::AddInvoked((invoke_id, Box::new(task))));
	}

	fn invoke(&mut self, invoke_id: TaskInvokeId) {
		self.task_action_queue
			.push(ScheduledTaskAction::Invoke(invoke_id));
	}

	#[inline]
	fn cancel_invoked(&mut self, invoke_id: TaskInvokeId) {
		self.task_action_queue
			.push(ScheduledTaskAction::CancelInvoked(invoke_id));
	}

	fn cancel(&mut self, owner_id: TaskCancellationId) {
		self.task_action_queue
			.push(ScheduledTaskAction::Cancel(owner_id));
	}

	fn generate_cancellation_id(&mut self) -> TaskCancellationId {
		self.task_cancellation_id_generator.get_next()
	}

	fn generate_invoke_id(&mut self) -> TaskInvokeId {
		self.task_invoke_id_generator.get_next()
	}
}
