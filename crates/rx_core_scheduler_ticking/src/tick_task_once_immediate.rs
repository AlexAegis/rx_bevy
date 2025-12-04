use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_traits::{
	Task, TaskContextProvider, Tick, TickResult, TickResultError, WithTaskInputOutput,
};

use crate::ExecuteTaskWorkOnce;

#[derive_where(Debug)]
pub struct ImmediateOnceTask<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnOnce(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Option<Work>,
	_phantom_data: PhantomData<fn(ContextProvider)>,
}

impl<Work, TaskResult, TaskError, ContextProvider> WithTaskInputOutput
	for ImmediateOnceTask<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnOnce(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	/// Technically this task will never need a tick, the scheduler that will use
	/// it will still want to give it one.
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
	type TaskResult = TaskResult;
	type TaskError = TaskError;
}

impl<Work, TaskResult, TaskError, ContextProvider> Task
	for ImmediateOnceTask<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnOnce(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	fn tick(
		&mut self,
		_tick: Self::TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<Self::TaskResult, Self::TaskError> {
		let Some(work) = self.work.take() else {
			return TickResult::Error(TickResultError::WorkAlreadyConsumed);
		};

		work.execute(context)
	}
}
