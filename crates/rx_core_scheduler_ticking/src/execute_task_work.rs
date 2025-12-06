use rx_core_traits::{TaskContextProvider, TickResult, TickResultError};

pub trait ExecuteTaskWorkMut<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(&mut self, context: &mut ContextProvider::Item<'_>) -> TickResult<TaskError>;
}

impl<F, TaskError, ContextProvider> ExecuteTaskWorkMut<TaskError, ContextProvider> for F
where
	F: FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError>,
	ContextProvider: TaskContextProvider,
{
	fn execute(&mut self, context: &mut ContextProvider::Item<'_>) -> TickResult<TaskError> {
		match (self)(context) {
			Ok(_) => TickResult::Done,
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}

pub trait ExecuteTaskWorkOnce<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(self, context: &mut ContextProvider::Item<'_>) -> TickResult<TaskError>;
}

impl<F, TaskError, ContextProvider> ExecuteTaskWorkOnce<TaskError, ContextProvider> for F
where
	F: FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError>,
	ContextProvider: TaskContextProvider,
{
	fn execute(self, context: &mut ContextProvider::Item<'_>) -> TickResult<TaskError> {
		match (self)(context) {
			Ok(_) => TickResult::Done,
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}
