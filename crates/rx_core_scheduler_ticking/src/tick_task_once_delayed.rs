use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_traits::{TaskContextProvider, TickResultError};

use rx_core_traits::{Task, Tick, TickResult, WithTaskInputOutput};

use crate::ExecuteTaskWorkOnce;

#[derive_where(Debug)]
pub struct DelayedOnceTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnOnce(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	elapsed: Duration,
	duration: Duration,
	#[derive_where(skip(Debug))]
	work: Option<Work>,
	_phantom_data: PhantomData<fn(ContextProvider)>,
}

impl<Work, TaskResult, TaskError, ContextProvider> WithTaskInputOutput
	for DelayedOnceTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnOnce(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
	type TaskResult = TaskResult;
	type TaskError = TaskError;
}

impl<Work, TaskResult, TaskError, ContextProvider> Task
	for DelayedOnceTaskTicked<Work, TaskResult, TaskError, ContextProvider>
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
		tick: Self::TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<Self::TaskResult, Self::TaskError> {
		self.elapsed += tick.delta;
		if self.duration <= self.elapsed {
			let Some(work) = self.work.take() else {
				return TickResult::Error(TickResultError::WorkAlreadyConsumed);
			};

			work.execute(context)
		} else {
			TickResult::Pending
		}
	}
}
