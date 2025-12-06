use std::time::Duration;

use derive_where::derive_where;
use rx_core_traits::{
	ScheduledTaskAction, SchedulerHandle, Task, TaskContextProvider, TaskExecutor, Tick,
	TickResult, WithTaskInputOutput,
};
use slab::Slab;

use crate::TickingScheduler;

#[derive_where(Default)]
pub struct TickingSchedulerExecutor<TaskError = (), ContextProvider = ()>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	current_tick: Tick,
	/// TODO: Instead of task id's use owner id's issued by schedulers, the tasks are going to be inside a slab, id'd, and a separate hashmap will store the owner_id/task_id map
	// ? If theres a finite type of tasks, we could just use multiple slabs of those specific types
	active_tasks: Slab<
		Box<
			dyn Task<TickInput = Tick, TaskError = TaskError, ContextProvider = ContextProvider>
				+ Send
				+ Sync,
		>,
	>,
	scheduler: SchedulerHandle<TickingScheduler<TaskError, ContextProvider>>,
}

impl<TaskError, ContextProvider> TickingSchedulerExecutor<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
}

impl<TaskError, ContextProvider> WithTaskInputOutput
	for TickingSchedulerExecutor<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	type TickInput = Tick;
	type TaskError = TaskError;
	type ContextProvider = ContextProvider;
}

impl<TaskError, ContextProvider> TaskExecutor
	for TickingSchedulerExecutor<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	type Scheduler = TickingScheduler<TaskError, ContextProvider>;

	#[inline]
	fn get_scheduler(&self) -> SchedulerHandle<Self::Scheduler> {
		self.scheduler.clone()
	}
}

impl<TaskError, ContextProvider> TaskExecutorWithManualTick
	for TickingSchedulerExecutor<TaskError, ContextProvider>
where
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync,
{
	fn tick(&mut self, tick: Tick, context: &mut ContextProvider::Item<'_>) {
		{
			let mut scheduler = self.scheduler.get_scheduler();
			for task_action in scheduler.drain_queue() {
				match task_action {
					ScheduledTaskAction::<Tick, TaskError, ContextProvider>::Activate((
						_task_id,
						task,
					)) => {
						self.active_tasks.insert(task);
					}
					ScheduledTaskAction::<Tick, TaskError, ContextProvider>::Cancel(task_id) => {
						self.active_tasks.remove(*task_id);
					}
				}
			}

			println!("UPDATING SSCHEDULERS CURRENT TICK TO {:?}", tick);
			scheduler.current_tick.update(tick);
		}
		self.current_tick.update(tick);

		// TODO, tasks may hold a reference to a subscriber, which holds a reference to the scheduler, so it's not much different than tasks receiving a scheduler reference on schedule.Meaning it could deadlock easily. Maybe there should be a task queue that is not locked during execution.
		// Tick until settled
		loop {
			let done_tasks = self
				.active_tasks
				.iter_mut()
				.filter_map(|(key, task)| {
					let task_result = task.tick(self.current_tick, context);
					// TODO: Do something with errors, maybe collect them and return? or define a handler and just pass them into? the tick fn should not have a return type.
					match task_result {
						TickResult::Done => Some(key),
						TickResult::Error(_error) => Some(key),
						TickResult::Pending => None,
					}
				})
				.collect::<Vec<_>>();

			println!("ticked scheduler {:?}, done tasks: {:?}", tick, done_tasks);

			if done_tasks.is_empty() {
				break;
			}

			for task_id in done_tasks {
				self.active_tasks.remove(task_id);
			}
		}
	}

	fn get_current_tick(&mut self) -> Tick {
		self.current_tick
	}
}

pub trait TaskExecutorWithManualTick: TaskExecutor {
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	);

	fn get_current_tick(&mut self) -> Tick;
}

pub trait SchedulerWithManualTickElapseExtension: TaskExecutorWithManualTick {
	fn tick_by_delta(
		&mut self,
		delta: Duration,
		context: &mut <Self::ContextProvider as TaskContextProvider>::Item<'_>,
	) {
		let next_tick = self.get_current_tick() + delta;
		self.tick(next_tick, context);
	}
}

impl<S> SchedulerWithManualTickElapseExtension for S where S: TaskExecutorWithManualTick {}
