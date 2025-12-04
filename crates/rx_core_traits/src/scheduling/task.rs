use std::fmt::Debug;

pub trait WithTaskInputOutput {
	/// Some schedulers pass inputs - such as the time passed - into the task
	/// to advance it.
	type TickInput;
	type TaskResult;
	type TaskError;
	type ContextProvider: TaskContextProvider;
}

pub trait TaskContextProvider {
	type Item<'c>;
}

impl TaskContextProvider for () {
	type Item<'c> = ();
}

pub trait Task: WithTaskInputOutput + Debug + Send + Sync {
	fn tick(
		&mut self,
		tick: Self::TickInput,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	) -> TickResult<Self::TaskResult, Self::TaskError>;
}

pub trait RepeatingTask: Task {
	/// Usually this is Self. Defining it like this preserves dyn compatibility
	type RepeatedTask: Task;

	fn repeat(&self) -> Self::RepeatedTask;
}

// TODO: Scheduled tasks must be cancellable if their owner unsubscribes, or make sure they drop if their target is closed, and they must only hold weak refs

// ? Maybe split TickResult to OnceTaskTickResult and RepeatingTaskTickResult
#[derive(Debug)]
pub enum TickResult<TaskResult, TaskError> {
	/// Not done yet
	Pending,
	/// Done with result
	Done(TaskResult),
	/// Done with error
	Error(TickResultError<TaskError>),
}

#[derive(Debug)]
pub enum TickResultError<TaskError> {
	WorkAlreadyConsumed,
	TaskError(TaskError),
}
