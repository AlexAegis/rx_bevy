use rx_core_traits::{
	ScheduledOnceWork, ScheduledRepeatedWork, TaskContextProvider, TickResult, TickResultError,
};

pub trait ExecuteTaskWorkMut<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(
		&mut self,
		tick_input: TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<TaskError>;
}

impl<F, TickInput, TaskError, ContextProvider>
	ExecuteTaskWorkMut<TickInput, TaskError, ContextProvider> for F
where
	F: ScheduledRepeatedWork<TickInput, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	fn execute(
		&mut self,
		tick_input: TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<TaskError> {
		match (self)(tick_input, context) {
			Ok(_) => TickResult::Done,
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}

pub trait ExecuteTaskWorkOnce<TickInput, TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	fn execute(
		self,
		tick_input: TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<TaskError>;
}

impl<F, TickInput, TaskError, ContextProvider>
	ExecuteTaskWorkOnce<TickInput, TaskError, ContextProvider> for F
where
	F: ScheduledOnceWork<TickInput, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	fn execute(
		self,
		tick_input: TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<TaskError> {
		match (self)(tick_input, context) {
			Ok(_) => TickResult::Done,
			Err(error) => TickResult::Error(TickResultError::TaskError(error)),
		}
	}
}
