use derive_where::derive_where;
use rx_core_traits::{
	ScheduledTaskAction, Scheduler, Task, TaskContextProvider, TaskId, Tick, WithTaskInputOutput,
};

use crate::{
	DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory, RepeatedTaskTickedFactory,
	TaskIdGenerator,
};

#[derive_where(Default)]
pub struct TickingScheduler<TaskError = (), ContextProvider = ()>
where
	ContextProvider: TaskContextProvider,
{
	task_id_generator: TaskIdGenerator,
	/// Updated by the executor at the start of each tick.
	pub(crate) current_tick: Tick,
	task_action_queue: Vec<ScheduledTaskAction<Tick, TaskError, ContextProvider>>,
	//_phantom_data: PhantomData<fn((TaskError, ContextProvider)) -> (TaskError, ContextProvider)>,
}

impl<TaskError, ContextProvider> TickingScheduler<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider + Send + Sync,
	TaskError: 'static,
{
	pub(crate) fn drain_queue(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, TaskError, ContextProvider>> {
		self.task_action_queue.drain(..)
	}
}

impl<TaskError, ContextProvider> WithTaskInputOutput
	for TickingScheduler<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider + Send + Sync,
{
	type TickInput = Tick;
	type TaskError = TaskError;
	type ContextProvider = ContextProvider;
}

impl<TaskError, ContextProvider> Scheduler for TickingScheduler<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<TaskError, ContextProvider>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<TaskError, ContextProvider>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<TaskError, ContextProvider>;

	fn schedule<T>(&mut self, mut task: T) -> TaskId
	where
		T: 'static
			+ Task<TickInput = Tick, TaskError = TaskError, ContextProvider = ContextProvider>
			+ Send
			+ Sync,
	{
		println!(
			"ON SCHEDULED HOOK, SCHEDULERS CURRENT TICK IS {:?}",
			self.current_tick
		);
		task.on_scheduled_hook(self.current_tick);
		let task_id = self.task_id_generator.get_next();
		self.task_action_queue
			.push(ScheduledTaskAction::Activate((task_id, Box::new(task))));
		task_id
	}

	fn cancel(&mut self, task_id: TaskId) {
		self.task_action_queue
			.push(ScheduledTaskAction::Cancel(task_id));
	}
}
