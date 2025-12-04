use std::time::Duration;

use thiserror::Error;

use crate::{Task, TaskContextProvider, TaskId, WithTaskInputOutput};

/// Schedulers define a set of tasks that can be offloaded to the scheduler to
/// be executed, and cancelled when no longer needed.
///
/// Store schedulers by a [SchedulerHandle]!
pub trait Scheduler: WithTaskInputOutput + Send + Sync {
	// Different types of tasks defined on the scheduler, so different  environments can create different ones, with just one subscriber
	// type DelayedTask: DelayedTaskFactory?;

	fn schedule<T>(&mut self, task: T) -> TaskId
	where
		T: 'static
			+ Task<
				TickInput = Self::TickInput,
				TaskResult = Self::TaskResult,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>;

	fn cancel(&mut self, task_id: TaskId) -> Result<(), TaskCancellationError>;
}

pub trait SchedulerWithManualTick: Scheduler {
	fn tick(
		&mut self,
		delta_time: Duration,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	);
}

#[derive(Error, Debug)]
pub enum TaskCancellationError {
	#[error("Task does not exist with id {0}!")]
	TaskDoesNotExist(TaskId),
}
