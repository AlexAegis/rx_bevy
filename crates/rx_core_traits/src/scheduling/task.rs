use std::{fmt::Debug, ops::AddAssign, time::Duration};

use crate::SubscriptionContext;

pub trait WithTaskInputOutput {
	/// Some schedulers pass inputs - such as the time passed - into the task
	/// to advance it.
	type TickInput;
	type TaskError;
	type ContextProvider: TaskContextProvider;
}

pub trait TaskContextProvider {
	type Item<'c>;
}

/// TODO: DELETE, Compat impl while SubContext still exists
impl<C> TaskContextProvider for C
where
	C: SubscriptionContext,
{
	type Item<'c> = C::Item<'c, 'c>;
}

// impl TaskContextProvider for () {
// 	type Item<'c> = ();
// }

pub trait Task: WithTaskInputOutput {
	fn tick(
		&mut self,
		tick_input: Self::TickInput,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	) -> TickResult<Self::TaskError>;

	/// The scheduler should calls this immediately when you pass the task into
	/// it, which happens before the first tick can.
	///
	/// TODO: VErify if it even makes sense or just defer to the next first tick
	/// on drain to act as initialize
	fn on_scheduled_hook(&mut self, tick_input: Self::TickInput);
}

// TODO: Scheduled tasks must be cancellable if their owner unsubscribes, or make sure they drop if their target is closed, and they must only hold weak refs

// ? Maybe split TickResult to OnceTaskTickResult and RepeatingTaskTickResult
#[derive(Debug)]
pub enum TickResult<TaskError> {
	/// Not done yet
	Pending,
	/// Done with result
	Done,

	/// Done with error
	Error(TickResultError<TaskError>),
}

#[derive(Debug)]
pub enum TickResultError<TaskError> {
	WorkAlreadyConsumed,
	TaskError(TaskError),
}

impl<TaskError> AddAssign for TickResult<TaskError> {
	fn add_assign(&mut self, rhs: Self) {
		let change = match self {
			TickResult::Pending => Some(rhs),
			TickResult::Done => match rhs {
				Self::Pending => None,
				_ => Some(rhs),
			},
			TickResult::Error(_) => match rhs {
				// TODO: Accumulate errors
				Self::Pending | Self::Done => None,
				_ => Some(rhs),
			},
		};

		if let Some(change) = change {
			*self = change;
		}
	}
}

pub trait ImmediateTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
}

pub trait RepeatedTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: 'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
}

pub trait DelayedTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
}

pub trait ImmediateTaskFactory<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	type Item<Work>: 'static
		+ Task<TickInput = TickInput, TaskError = TaskError, ContextProvider = ContextProvider>
		+ Send
		+ Sync
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;
}

pub trait RepeatedTaskFactory<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	type Item<Work>: 'static
		+ Task<TickInput = TickInput, TaskError = TaskError, ContextProvider = ContextProvider>
		+ Send
		+ Sync
	where
		Work:
			'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work:
			'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;
}

pub trait DelayedTaskFactory<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	type Item<Work>: 'static
		+ Task<TickInput = TickInput, TaskError = TaskError, ContextProvider = ContextProvider>
		+ Send
		+ Sync
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;

	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;
}
