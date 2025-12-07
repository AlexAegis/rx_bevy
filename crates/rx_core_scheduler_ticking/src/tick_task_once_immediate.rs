use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_traits::{
	ImmediateTaskFactory, ScheduledOnceWork, Task, TaskContextProvider, Tick, TickResult,
	TickResultError, WithTaskInputOutput,
};

use crate::ExecuteTaskWorkOnce;

pub struct ImmediateOnceTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	_phantom_data: PhantomData<(TaskError, ContextProvider)>,
}

impl<TaskError, ContextProvider> ImmediateTaskFactory<Tick, TaskError, ContextProvider>
	for ImmediateOnceTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider,
	TaskError: 'static,
{
	type Item<Work>
		= ImmediateOnceTaskTicked<Work, TaskError, ContextProvider>
	where
		Work: ScheduledOnceWork<Tick, TaskError, ContextProvider>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<Tick, TaskError, ContextProvider>,
	{
		ImmediateOnceTaskTicked {
			work: Some(work),
			_phantom_data: PhantomData,
		}
	}
}

#[derive_where(Debug)]
pub struct ImmediateOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: ScheduledOnceWork<Tick, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Option<Work>,
	_phantom_data: PhantomData<fn((ContextProvider, TaskError)) -> (ContextProvider, TaskError)>,
}

impl<Work, TaskError, ContextProvider> WithTaskInputOutput
	for ImmediateOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: ScheduledOnceWork<Tick, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	/// Technically this task will never need a tick, the scheduler that will use
	/// it will still want to give it one.
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
	type TaskError = TaskError;
}

impl<Work, TaskError, ContextProvider> Task
	for ImmediateOnceTaskTicked<Work, TaskError, ContextProvider>
where
	Work: ScheduledOnceWork<Tick, TaskError, ContextProvider>,
	ContextProvider: TaskContextProvider,
{
	fn tick(
		&mut self,
		tick: Tick,
		task_input: &mut ContextProvider::Item<'_>,
	) -> TickResult<Self::TaskError> {
		let Some(work) = self.work.take() else {
			return TickResult::Error(TickResultError::WorkAlreadyConsumed);
		};

		work.execute(tick, task_input)
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::TickInput) {}
}
