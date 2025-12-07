use std::{fmt::Debug, time::Duration};

use rx_core_traits::{
	ScheduledTaskAction, Scheduler, SchedulerHandle, Task, TaskContextProvider, TaskExecutor,
	TaskOwnerId, Tick, TickResult, WithTaskInputOutput,
};
use slab::Slab;

pub trait TickingExecutorsScheduler: Scheduler {
	fn update_tick(&mut self, tick: Tick);

	fn drain_queue(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, Self::TaskError, Self::ContextProvider>>;
}

pub struct TickingSchedulerExecutor<S, TaskError = (), ContextProvider = ()>
where
	S: Scheduler<TickInput = Tick, TaskError = TaskError, ContextProvider = ContextProvider>,
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync + Debug,
{
	current_tick: Tick,
	// TODO: This could store an enum instead of a dyn object
	active_tasks: Slab<(
		TaskOwnerId,
		Box<
			dyn Task<TickInput = Tick, TaskError = TaskError, ContextProvider = ContextProvider>
				+ Send
				+ Sync,
		>,
	)>,
	scheduler: SchedulerHandle<S>,
}

impl<S, TaskError, ContextProvider> TickingSchedulerExecutor<S, TaskError, ContextProvider>
where
	S: TickingExecutorsScheduler<
			TickInput = Tick,
			TaskError = TaskError,
			ContextProvider = ContextProvider,
		>,
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync + Debug,
{
	pub fn new(scheduler: S) -> Self {
		Self {
			current_tick: Tick::default(),
			active_tasks: Slab::new(),
			scheduler: SchedulerHandle::new(scheduler),
		}
	}

	pub fn get_current_tick(&mut self) -> Tick {
		self.current_tick
	}

	pub fn tick(&mut self, tick: Tick, context: &mut ContextProvider::Item<'_>) {
		loop {
			self.drain_scheduler_queue(tick);

			let done_tasks = self.tick_tasks(tick, context);

			if done_tasks.is_empty() {
				break;
			}

			for task_id in done_tasks
				.iter()
				.filter_map(|task_id_to_remove| *task_id_to_remove)
			{
				self.active_tasks.remove(task_id);
			}
		}
	}

	pub fn tick_by_delta(
		&mut self,
		delta: Duration,
		context: &mut <ContextProvider as TaskContextProvider>::Item<'_>,
	) {
		let next_tick = self.get_current_tick() + delta;
		self.tick(next_tick, context);
	}

	fn drain_scheduler_queue(&mut self, tick: Tick) {
		let mut scheduler = self.scheduler.get_scheduler();
		scheduler.update_tick(tick);

		for task_action in scheduler.drain_queue() {
			match task_action {
				ScheduledTaskAction::<Tick, TaskError, ContextProvider>::Activate((
					owner_id,
					task,
				)) => {
					self.active_tasks.insert((owner_id, task));
				}
				ScheduledTaskAction::CancelAll(owner_id) => {
					self.active_tasks
						.retain(|_task_id, (task_owner_id, _task)| task_owner_id != &owner_id);
				}
			}
		}
	}

	fn tick_tasks(
		&mut self,
		tick: Tick,
		context: &mut ContextProvider::Item<'_>,
	) -> Vec<Option<usize>> {
		self.current_tick.update(tick);
		self.active_tasks
			.iter_mut()
			.filter_map(|(key, (_owner_id, task))| {
				let task_result = task.tick(self.current_tick, context);
				// TODO: Do something with errors, maybe collect them and return? or define a handler and just pass them into? the tick fn should not have a return type.
				match task_result {
					TickResult::Done => Some(Some(key)),
					TickResult::Error(_error) => Some(Some(key)),
					TickResult::Dirty => Some(None),
					TickResult::Pending => None,
				}
			})
			.collect::<Vec<_>>()
	}
}

impl<S, TaskError, ContextProvider> WithTaskInputOutput
	for TickingSchedulerExecutor<S, TaskError, ContextProvider>
where
	S: TickingExecutorsScheduler<
			TickInput = Tick,
			TaskError = TaskError,
			ContextProvider = ContextProvider,
		>,
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync + Debug,
{
	type TickInput = Tick;
	type TaskError = TaskError;
	type ContextProvider = ContextProvider;
}

impl<S, TaskError, ContextProvider> TaskExecutor
	for TickingSchedulerExecutor<S, TaskError, ContextProvider>
where
	S: TickingExecutorsScheduler<
			TickInput = Tick,
			TaskError = TaskError,
			ContextProvider = ContextProvider,
		>,
	ContextProvider: 'static + TaskContextProvider + Send + Sync,
	TaskError: 'static + Send + Sync + Debug,
{
	type Scheduler = S;

	#[inline]
	fn get_scheduler(&self) -> SchedulerHandle<Self::Scheduler> {
		self.scheduler.clone()
	}
}
