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
	type Item<'c>: TaskContextItem<'c>;
}

pub trait TaskContextItem<'c> {
	fn now(&self) -> Duration;
}

/// TODO: DELETE, Compat impl while SubContext still exists
impl<C> TaskContextProvider for C
where
	C: SubscriptionContext,
	for<'c> C::Item<'c, 'c>: TaskContextItem<'c>,
{
	type Item<'c> = C::Item<'c, 'c>;
}

/*
impl<C> SubscriptionContext for C
where
	C: TaskContextProvider,
{
	type Item<'w, 's> = C::Item<'w>;
}
*/
/*impl TaskContextProvider for () {
	type Item<'c> = ();
}*/

impl<'c> TaskContextItem<'c> for () {
	fn now(&self) -> Duration {
		Duration::from_millis(0)
	}
}

// pub struct TaskInput<'a, TickInput, ContextProvider>
// where
// 	ContextProvider: TaskContextProvider,
// {
// 	pub tick_input: TickInput,
// 	pub context: &'a mut ContextProvider::Item<'a>,
// }

pub trait Task: WithTaskInputOutput {
	fn tick(
		&mut self,
		task_input: Self::TickInput,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	) -> TickResult<Self::TaskError>;

	/// The scheduler should calls this immediately when you pass the task into
	/// it, which happens before the first tick can.
	///
	/// TODO: VErify if it even makes sense or just defer to the next first tick
	/// on drain to act as initialize
	///
	/// TODO: ADD A RETURN VALUE AND RETURN IT TO THE USER, BUT ONLY MAKES SENSE WITH THE CONTEXT, BUT THAT CAN'T BE CALLED??
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
	/// Not done, but a new job may have been scheduled
	Dirty,
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
			Self::Pending => Some(rhs),
			Self::Dirty => match rhs {
				Self::Pending => None,
				_ => Some(rhs),
			},
			Self::Done => match rhs {
				Self::Pending | Self::Dirty => None,
				_ => Some(rhs),
			},
			Self::Error(_) => match rhs {
				// TODO: Accumulate errors
				Self::Pending | Self::Dirty | Self::Done => None,
				_ => Some(rhs),
			},
		};

		if let Some(change) = change {
			*self = change;
		}
	}
}

pub trait ScheduledOnceWork<TickInput, TaskError, ContextProvider>:
	'static + FnOnce(TickInput, &mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync
where
	ContextProvider: TaskContextProvider,
{
}

impl<W, TickInput, TaskError, ContextProvider>
	ScheduledOnceWork<TickInput, TaskError, ContextProvider> for W
where
	ContextProvider: TaskContextProvider,
	W: 'static
		+ FnOnce(TickInput, &mut ContextProvider::Item<'_>) -> Result<(), TaskError>
		+ Send
		+ Sync,
{
}

pub trait ScheduledRepeatedWork<TickInput, TaskError, ContextProvider>:
	'static + FnMut(TickInput, &mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync
where
	ContextProvider: TaskContextProvider,
{
}

impl<W, TickInput, TaskError, ContextProvider>
	ScheduledRepeatedWork<TickInput, TaskError, ContextProvider> for W
where
	ContextProvider: TaskContextProvider,
	W: 'static
		+ FnMut(TickInput, &mut ContextProvider::Item<'_>) -> Result<(), TaskError>
		+ Send
		+ Sync,
{
}

pub trait ImmediateTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
}

pub trait RepeatedTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: ScheduledRepeatedWork<TickInput, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
}

pub trait DelayedTask<Work, TickInput, TaskError, ContextProvider>: Task
where
	Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>,
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
		Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>;
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
		Work: ScheduledRepeatedWork<TickInput, TaskError, ContextProvider>;

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, TaskError, ContextProvider>;
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
		Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>;

	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, TaskError, ContextProvider>;
}
