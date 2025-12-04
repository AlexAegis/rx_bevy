use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_traits::{
	RepeatingTask, Task, TaskContextProvider, Tick, TickResult, WithTaskInputOutput,
};

use crate::ExecuteTaskWorkMut;

#[derive_where(Debug)]
pub struct DelayedRepeatingTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnMut(
			&mut <ContextProvider as TaskContextProvider>::Item<'_>,
		) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	start_immediately: bool,
	elapsed: Duration,
	duration: Duration,
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(ContextProvider)>,
}

impl<Work, TaskResult, TaskError, ContextProvider>
	DelayedRepeatingTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnMut(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
}

impl<Work, TaskResult, TaskError, ContextProvider> WithTaskInputOutput
	for DelayedRepeatingTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnMut(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
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
	for DelayedRepeatingTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnMut(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ ExecuteTaskWorkMut<TaskResult, TaskError, ContextProvider>
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
		if self.start_immediately {
			self.start_immediately = false;
			return self.work.execute(context);
		}

		// TODO: Maybe instead of a separate repeating type, if repeating is true in the normal delayed on, just reschedule self, give tick the scheduler reference
		self.elapsed += tick.delta;
		if self.duration <= self.elapsed {
			self.work.execute(context)
		} else {
			TickResult::Pending
		}
	}
}

impl<Work, TaskResult, TaskError, ContextProvider> RepeatingTask
	for DelayedRepeatingTaskTicked<Work, TaskResult, TaskError, ContextProvider>
where
	Work: 'static
		+ FnMut(&mut ContextProvider::Item<'_>) -> Result<TaskResult, TaskError>
		+ Clone
		+ Send
		+ Sync,
	ContextProvider: TaskContextProvider,
{
	type RepeatedTask = Self;

	fn repeat(&self) -> Self::RepeatedTask {
		Self {
			start_immediately: false,
			duration: self.duration,
			// In case of a big tick, immediately Done tasks will also repeat
			// immediately by the scheduler, so only subtracting one duration
			// here is important
			elapsed: self.elapsed - self.duration,
			work: self.work.clone(),
			_phantom_data: PhantomData,
		}
	}
}
