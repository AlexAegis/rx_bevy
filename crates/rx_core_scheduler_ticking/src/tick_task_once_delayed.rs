use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_traits::{DelayedTask, DelayedTaskFactory, TaskContextProvider, TickResultError};

use rx_core_traits::{Task, Tick, TickResult, WithTaskInputOutput};

use crate::ExecuteTaskWorkOnce;

pub struct DelayedOnceTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	_phantom_data: PhantomData<(TaskError, ContextProvider)>,
}

impl<TaskError, ContextProvider> DelayedTaskFactory<Tick, TaskError, ContextProvider>
	for DelayedOnceTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	type Item<Work>
		= DelayedOnceTaskTicked<Work, TaskError, ContextProvider>
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;
	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work:
			'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	{
		DelayedOnceTaskTicked {
			work: Some(work),
			current_tick: Tick::default(),
			scheduled_on: Tick::default(),
			delay,
			_phantom_data: PhantomData,
		}
	}
}

#[derive_where(Debug)]
pub struct DelayedOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	scheduled_on: Tick,
	current_tick: Tick,
	delay: Duration,

	#[derive_where(skip(Debug))]
	work: Option<Work>,

	_phantom_data: PhantomData<(TaskError, ContextProvider)>,
}

impl<Work, TaskError, ContextProvider> WithTaskInputOutput
	for DelayedOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
	type TaskError = TaskError;
}

impl<Work, TaskError, ContextProvider> DelayedTask<Work, Tick, TaskError, ContextProvider>
	for DelayedOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider + Send + Sync,
	TaskError: Send + Sync,
{
}

impl<Work, TaskError, ContextProvider> Task
	for DelayedOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnOnce(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	fn tick(
		&mut self,
		tick: Self::TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<Self::TaskError> {
		self.current_tick.update(tick);
		// TODO: it should take into account when the task started
		if self.scheduled_on + self.delay <= self.current_tick {
			let Some(work) = self.work.take() else {
				return TickResult::Error(TickResultError::WorkAlreadyConsumed);
			};

			work.execute(context)
		} else {
			TickResult::Pending
		}
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::TickInput) {
		self.scheduled_on.update(tick_input);
		self.current_tick.update(tick_input);
	}
}
