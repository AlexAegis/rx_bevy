use std::time::Duration;

use crate::{
	DelayedTaskFactory, ImmediateTaskFactory, RepeatedTaskFactory, ScheduledOnceWork,
	ScheduledRepeatedWork, SchedulerHandle, Task, TaskContextProvider, TaskOwnerId,
	WithTaskInputOutput,
};

/// Schedulers define a set of tasks that can be offloaded to the scheduler to
/// be executed, and cancelled when no longer needed.
///
/// Store schedulers by a [SchedulerHandle]!
pub trait Scheduler: WithTaskInputOutput {
	// Different types of tasks defined on the scheduler, so different  environments can create different ones, with just one subscriber
	// type DelayedTask: DelayedTaskFactory?;

	type DelayedTaskFactory: DelayedTaskFactory<Self::TickInput, Self::TaskError, Self::ContextProvider>;
	type RepeatedTaskFactory: RepeatedTaskFactory<Self::TickInput, Self::TaskError, Self::ContextProvider>;
	type ImmediateTaskFactory: ImmediateTaskFactory<Self::TickInput, Self::TaskError, Self::ContextProvider>;

	fn schedule<T>(&mut self, task: T, owner_id: TaskOwnerId)
	where
		T: 'static
			+ Task<
				TickInput = Self::TickInput,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>
			+ Send
			+ Sync;

	fn cancel(&mut self, owner_id: TaskOwnerId);

	fn generate_owner_id(&mut self) -> TaskOwnerId;
}

pub enum ScheduledTaskAction<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	Activate(
		(
			TaskOwnerId,
			Box<
				dyn Task<
						TickInput = TickInput,
						TaskError = TaskError,
						ContextProvider = ContextProvider,
					> + Send
					+ Sync,
			>,
		),
	),
	CancelAll(TaskOwnerId),
}

pub trait TaskExecutor: WithTaskInputOutput {
	type Scheduler: Scheduler<
			TickInput = Self::TickInput,
			TaskError = Self::TaskError,
			ContextProvider = Self::ContextProvider,
		>;

	fn get_scheduler(&self) -> SchedulerHandle<Self::Scheduler>;
}

pub trait SchedulerScheduleTaskExtension: Scheduler {
	fn schedule_delayed_task<Work>(&mut self, work: Work, delay: Duration, owner_id: TaskOwnerId)
	where
		Work: ScheduledOnceWork<Self::TickInput, Self::TaskError, Self::ContextProvider>,
	{
		self.schedule(Self::DelayedTaskFactory::new(work, delay), owner_id)
	}

	fn schedule_repeated_task<Work>(
		&mut self,
		work: Work,
		interval: Duration,
		start_immediately: bool,
		owner_id: TaskOwnerId,
	) where
		Work: ScheduledRepeatedWork<Self::TickInput, Self::TaskError, Self::ContextProvider>,
	{
		self.schedule(
			Self::RepeatedTaskFactory::new(work, interval, start_immediately),
			owner_id,
		)
	}

	fn schedule_immediate_task<Work>(&mut self, work: Work, owner_id: TaskOwnerId)
	where
		Work: ScheduledOnceWork<Self::TickInput, Self::TaskError, Self::ContextProvider>,
	{
		self.schedule(Self::ImmediateTaskFactory::new(work), owner_id)
	}
}

impl<S> SchedulerScheduleTaskExtension for S where S: Scheduler {}
