use std::time::Duration;

use crate::{
	DelayedTaskFactory, ImmediateTaskFactory, RepeatedTaskFactory, SchedulerHandle, Task,
	TaskContextProvider, TaskId, WithTaskInputOutput,
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

	fn schedule<T>(&mut self, task: T) -> TaskId
	where
		T: 'static
			+ Task<
				TickInput = Self::TickInput,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>
			+ Send
			+ Sync;

	fn cancel(&mut self, task_id: TaskId);
}

pub enum ScheduledTaskAction<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	Activate(
		(
			TaskId,
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
	Cancel(TaskId),
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
	fn schedule_delayed_task<Work>(&mut self, work: Work, delay: Duration) -> TaskId
	where
		Work: 'static
			+ FnOnce(
				&mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
			) -> Result<(), Self::TaskError>
			+ Send
			+ Sync,
	{
		self.schedule(Self::DelayedTaskFactory::new(work, delay))
	}

	fn schedule_repeated_task<Work>(
		&mut self,
		work: Work,
		interval: Duration,
		start_immediately: bool,
	) -> TaskId
	where
		Work: 'static
			+ FnMut(
				&mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
			) -> Result<(), Self::TaskError>
			+ Send
			+ Sync,
	{
		self.schedule(Self::RepeatedTaskFactory::new(
			work,
			interval,
			start_immediately,
		))
	}

	fn schedule_immediate_task<Work>(&mut self, work: Work) -> TaskId
	where
		Work: 'static
			+ FnOnce(
				&mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
			) -> Result<(), Self::TaskError>
			+ Send
			+ Sync,
	{
		self.schedule(Self::ImmediateTaskFactory::new(work))
	}
}

impl<S> SchedulerScheduleTaskExtension for S where S: Scheduler {}
