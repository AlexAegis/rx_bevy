use std::fmt::Debug;

use derive_where::derive_where;
use rx_core_traits::{
	ScheduledTaskAction, Scheduler, Task, TaskContextProvider, TaskOwnerId, TaskOwnerIdGenerator,
	Tick, WithTaskInputOutput,
};

use crate::{
	DelayedOnceTaskTickedFactory, ImmediateOnceTaskTickedFactory, RepeatedTaskTickedFactory,
	TickingExecutorsScheduler,
};

#[derive_where(Default)]
pub struct TickingScheduler<TaskError = (), ContextProvider = ()>
where
	ContextProvider: TaskContextProvider,
{
	task_owner_id_generator: TaskOwnerIdGenerator,
	/// Updated by the executor at the start of each tick.
	pub(crate) current_tick: Tick,
	task_action_queue: Vec<ScheduledTaskAction<Tick, TaskError, ContextProvider>>,
	//_phantom_data: PhantomData<fn((TaskError, ContextProvider)) -> (TaskError, ContextProvider)>,
}

impl<TaskError, ContextProvider> TickingExecutorsScheduler
	for TickingScheduler<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync + Debug,
{
	fn drain_queue(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, TaskError, ContextProvider>> {
		self.task_action_queue.drain(..)
	}

	fn update_tick(&mut self, tick: Tick) {
		self.current_tick.update(tick);
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
	TaskError: 'static + Send + Sync + Debug,
{
	type DelayedTaskFactory = DelayedOnceTaskTickedFactory<TaskError, ContextProvider>;
	type ImmediateTaskFactory = ImmediateOnceTaskTickedFactory<TaskError, ContextProvider>;
	type RepeatedTaskFactory = RepeatedTaskTickedFactory<TaskError, ContextProvider>;

	fn schedule<T>(&mut self, mut task: T, owner_id: TaskOwnerId)
	where
		T: 'static
			+ Task<TickInput = Tick, TaskError = TaskError, ContextProvider = ContextProvider>
			+ Send
			+ Sync,
	{
		task.on_scheduled_hook(self.current_tick);

		self.task_action_queue
			.push(ScheduledTaskAction::Activate((owner_id, Box::new(task))));

		//	let mut s = SubscriptionData::<ContextProvider>::default();
		// TODO: Try returning subscriptions instead of ownerids
	}

	fn cancel(&mut self, owner_id: TaskOwnerId) {
		self.task_action_queue
			.push(ScheduledTaskAction::CancelAll(owner_id));
	}

	fn generate_owner_id(&mut self) -> TaskOwnerId {
		self.task_owner_id_generator.get_next()
	}
}
