use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_traits::{
	RepeatedTaskFactory, Task, TaskContextProvider, Tick, TickResult, WithTaskInputOutput,
};

use crate::ExecuteTaskWorkMut;

pub struct RepeatedTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: TaskContextProvider,
{
	_phantom_data: PhantomData<(TaskError, ContextProvider)>,
}

impl<TaskError, ContextProvider> RepeatedTaskFactory<Tick, TaskError, ContextProvider>
	for RepeatedTaskTickedFactory<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider,
	TaskError: 'static,
{
	type Item<Work>
		= DelayedRepeatingTaskTicked<Work, TaskError, ContextProvider>
	where
		Work:
			'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync;

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work:
			'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	{
		DelayedRepeatingTaskTicked {
			start_immediately,
			consumed_until: Tick::default(),
			current_tick: Tick::default(),
			interval,
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive_where(Debug)]
pub struct DelayedRepeatingTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	/// The work will be executed on the first tick too, regardless if the timer
	/// had elapsed or not.
	start_immediately: bool,
	consumed_until: Tick,
	current_tick: Tick,
	interval: Duration,
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn((TaskError, ContextProvider)) -> (TaskError, ContextProvider)>,
}

impl<Work, TaskError, ContextProvider> WithTaskInputOutput
	for DelayedRepeatingTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	type TickInput = Tick;
	type ContextProvider = ContextProvider;
	type TaskError = TaskError;
}

impl<Work, TaskError, ContextProvider> Task
	for DelayedRepeatingTaskTicked<Work, TaskError, ContextProvider>
where
	Work: 'static + FnMut(&mut ContextProvider::Item<'_>) -> Result<(), TaskError> + Send + Sync,
	ContextProvider: TaskContextProvider,
{
	fn tick(
		&mut self,
		tick: Self::TickInput,
		context: &mut ContextProvider::Item<'_>,
	) -> TickResult<Self::TaskError> {
		if self.start_immediately {
			self.start_immediately = false;
			return self.work.execute(context);
		}
		self.current_tick.update(tick);

		let mut tick_result = TickResult::Pending;
		while self.consumed_until + self.interval <= self.current_tick {
			self.consumed_until += self.interval;
			tick_result += self.work.execute(context);
		}
		tick_result
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::TickInput) {
		self.consumed_until.update(tick_input);
		self.current_tick.update(tick_input);
	}
}
