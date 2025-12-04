use std::{marker::PhantomData, time::Duration};

use rx_core_traits::{
	Scheduler, SchedulerWithManualTick, Task, TaskCancellationError, TaskContextProvider, TaskId,
	Tick, TickResult, WithTaskInputOutput,
};
use slab::Slab;

use crate::TickIndexGenerator;

#[derive(Default, Debug)]
pub struct TickingScheduler<ContextProvider = (), TaskResult = (), TaskError = ()>
where
	ContextProvider: TaskContextProvider + Send + Sync,
{
	tick_index_generator: TickIndexGenerator,
	elapsed: Duration,
	// ? If theres a finite type of tasks, we could just use multiple slabs of those specific types
	active_tasks: Slab<
		Box<
			dyn Task<
					TickInput = <Self as WithTaskInputOutput>::TickInput,
					TaskResult = <Self as WithTaskInputOutput>::TaskResult,
					TaskError = <Self as WithTaskInputOutput>::TaskError,
					ContextProvider = <Self as WithTaskInputOutput>::ContextProvider,
				>,
		>,
	>,
	_phantom_data: PhantomData<ContextProvider>,
}

impl<ContextProvider, TaskResult, TaskError> WithTaskInputOutput
	for TickingScheduler<ContextProvider, TaskResult, TaskError>
where
	ContextProvider: TaskContextProvider + Send + Sync,
{
	type TickInput = Tick;
	type TaskResult = TaskResult;
	type TaskError = TaskError;
	type ContextProvider = ContextProvider;
}

impl<ContextProvider, TaskResult, TaskError> Scheduler
	for TickingScheduler<ContextProvider, TaskResult, TaskError>
where
	ContextProvider: TaskContextProvider + Send + Sync,
{
	fn schedule<T>(&mut self, task: T) -> TaskId
	where
		T: 'static
			+ Task<
				TickInput = Self::TickInput,
				TaskResult = Self::TaskResult,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>,
	{
		self.active_tasks.insert(Box::new(task)).into()
	}

	fn cancel(&mut self, task_id: TaskId) -> Result<(), TaskCancellationError> {
		self.active_tasks
			.try_remove(*task_id)
			.map(|_| ())
			.ok_or(TaskCancellationError::TaskDoesNotExist(task_id))
	}
}

impl<ContextProvider, TaskResult, TaskError> SchedulerWithManualTick
	for TickingScheduler<ContextProvider, TaskResult, TaskError>
where
	ContextProvider: TaskContextProvider + Send + Sync,
{
	fn tick(&mut self, delta_time: Duration, context: &mut ContextProvider::Item<'_>) {
		self.elapsed += delta_time;

		let tick = Tick {
			index: *self.tick_index_generator.get_next(),
			delta: delta_time,
			now: self.elapsed,
		};

		let done_tasks = self
			.active_tasks
			.iter_mut()
			.filter_map(|(key, task)| {
				let task_result = task.tick(tick.clone(), context);
				match task_result {
					TickResult::Done(_result) => Some(key),
					TickResult::Error(_error) => Some(key),
					TickResult::Pending => None,
				}
			})
			.collect::<Vec<_>>();

		// TODO: Repeatedly drain by ticking with 0 long ticks until there is nothing done. increment tick index

		for task_id in done_tasks {
			self.active_tasks.remove(task_id);
		}
	}
}
