use rx_core_traits::{TaskContextProvider, TickResult, TickResultError};

pub trait ExecuteTaskWorkMut<TaskResult, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(
		&mut self,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<TaskResult, TaskError>;
}

impl<F, TaskResult, TaskError, ContextProvider>
	ExecuteTaskWorkMut<TaskResult, TaskError, ContextProvider> for F
where
	F: FnMut(
		&mut <ContextProvider as TaskContextProvider>::Item<'_>,
	) -> Result<TaskResult, TaskError>,
	ContextProvider: TaskContextProvider,
{
	fn execute(
		&mut self,
		context: &mut <ContextProvider as TaskContextProvider>::Item<'_>,
	) -> TickResult<TaskResult, TaskError> {
		match (self)(context) {
			Ok(result) => TickResult::Done(result),
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}

pub trait ExecuteTaskWorkOnce<TaskResult, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(self, context: &mut ContextProvider::Item<'_>) -> TickResult<TaskResult, TaskError>;
}

impl<F, TaskResult, TaskError, ContextProvider>
	ExecuteTaskWorkOnce<TaskResult, TaskError, ContextProvider> for F
where
	F: FnOnce(
		&mut <ContextProvider as TaskContextProvider>::Item<'_>,
	) -> Result<TaskResult, TaskError>,
	ContextProvider: TaskContextProvider,
{
	fn execute(
		self,
		context: &mut <ContextProvider as TaskContextProvider>::Item<'_>,
	) -> TickResult<TaskResult, TaskError> {
		match (self)(context) {
			Ok(result) => TickResult::Done(result),
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}
