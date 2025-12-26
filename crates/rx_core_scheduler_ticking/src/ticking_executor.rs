use std::{
	collections::{HashMap, HashSet},
	time::Duration,
};

use rx_core_macro_executor_derive::RxExecutor;
use rx_core_traits::{
	ContextProvider, ScheduledTaskAction, Scheduler, SchedulerHandle, Task, TaskCancellationId,
	TaskInvokeId, TaskResult,
};
use slab::Slab;

use crate::Tick;

pub trait TickingExecutorsScheduler: Scheduler {
	fn update_tick(&mut self, tick: Tick);

	fn drain_tasks(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledTaskAction<Tick, Self::ContextProvider>>;

	/// Returns true if there are tasks queued.
	fn has_tasks(&self) -> bool;
}

#[derive(RxExecutor)]
#[rx_context(C)]
#[rx_tick(Tick)]
#[rx_scheduler(S)]
pub struct TickingSchedulerExecutor<S, C>
where
	S: Scheduler<Tick = Tick, ContextProvider = C>,
	C: 'static + ContextProvider + Send + Sync,
{
	#[scheduler_handle]
	scheduler: SchedulerHandle<S>,

	current_tick: Tick,

	// TODO: ADD FROM THE BEVY SIDE SOME NEW TASK TYPES LIKE SEND EVENT, THAT NEEDS NO CALLBACKS
	active_tasks: Slab<(
		TaskCancellationId,
		Box<dyn Task<Tick = Tick, ContextProvider = C> + Send + Sync>,
	)>,
	invokable_tasks:
		HashMap<TaskInvokeId, Box<dyn Task<Tick = Tick, ContextProvider = C> + Send + Sync>>,
	invoked: Vec<TaskInvokeId>,
	tasks_already_ticked: HashSet<usize>,
}

impl<S, C> TickingSchedulerExecutor<S, C>
where
	S: TickingExecutorsScheduler<Tick = Tick, ContextProvider = C>,
	C: 'static + ContextProvider + Send + Sync,
{
	pub fn new(scheduler: S) -> Self {
		Self {
			current_tick: Tick::default(),
			active_tasks: Slab::new(),
			invokable_tasks: HashMap::new(),
			invoked: Vec::new(),
			scheduler: SchedulerHandle::new(scheduler),
			tasks_already_ticked: HashSet::new(),
		}
	}

	pub fn now(&self) -> Duration {
		self.current_tick.elapsed_since_start
	}

	/// Returns `true` when there are no active tasks in the executor.
	pub fn is_empty(&self) -> bool {
		self.active_tasks.is_empty()
	}

	pub fn get_current_tick(&self) -> Tick {
		self.current_tick
	}

	pub fn tick_to(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		self.current_tick.update(tick);
		self.tasks_already_ticked.clear();
		loop {
			self.drain_scheduler_queue(tick);

			let no_tasks_ticked = self.tick_scheduled_tasks(tick, context);

			self.execute_invoked_tasks(tick, context);

			if no_tasks_ticked {
				break;
			}
		}
	}

	pub fn tick(&mut self, delta: Duration, context: &mut C::Item<'_>) {
		let next_tick = self.get_current_tick() + delta;
		self.tick_to(next_tick, context);
	}

	fn execute_invoked_tasks(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		for invoked_task_id in self.invoked.drain(..) {
			if let Some(invoked_task) = self.invokable_tasks.get_mut(&invoked_task_id) {
				let invoke_result = invoked_task.tick(tick, context);
				if matches!(invoke_result, TaskResult::Done) {
					self.invokable_tasks.remove(&invoked_task_id);
				}
			}
		}
	}

	fn drain_scheduler_queue(&mut self, tick: Tick) {
		let mut scheduler = self.scheduler.lock();
		scheduler.update_tick(tick);

		for task_action in scheduler.drain_tasks() {
			match task_action {
				ScheduledTaskAction::<Tick, C>::Activate((owner_id, task)) => {
					self.active_tasks.insert((owner_id, task));
				}
				ScheduledTaskAction::AddInvoked((invoke_id, task)) => {
					self.invokable_tasks.insert(invoke_id, task);
				}
				ScheduledTaskAction::Invoke(invoke_id) => self.invoked.push(invoke_id),
				ScheduledTaskAction::CancelInvoked(invoke_id) => {
					self.invokable_tasks.remove(&invoke_id);
					self.invoked.retain(|invoked_id| invoked_id == &invoke_id);
				}
				ScheduledTaskAction::Cancel(owner_id) => {
					self.active_tasks
						.retain(|_task_id, (task_owner_id, _task)| task_owner_id != &owner_id);
				}
			}
		}
	}

	fn tick_scheduled_tasks(&mut self, tick: Tick, context: &mut C::Item<'_>) -> bool {
		let mut tasks_done = Vec::<usize>::new();
		let mut tasks_ticked = Vec::<usize>::new();

		for (key, (_, task)) in self
			.active_tasks
			.iter_mut()
			.filter(|(key, _)| !self.tasks_already_ticked.contains(key))
		{
			let task_result = task.tick(tick, context);

			if matches!(task_result, TaskResult::Done) {
				tasks_done.push(key);
			}

			tasks_ticked.push(key);
		}

		for task_key in tasks_done {
			self.active_tasks.remove(task_key);
		}

		let no_tasks_ticked = tasks_ticked.is_empty();

		for task_key in tasks_ticked {
			self.tasks_already_ticked.insert(task_key);
		}

		no_tasks_ticked
	}
}
