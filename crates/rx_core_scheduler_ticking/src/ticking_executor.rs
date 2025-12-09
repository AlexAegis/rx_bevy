use std::{collections::HashMap, time::Duration};

use rx_core_macro_executor_derive::RxExecutor;
use rx_core_traits::{
	ContextProvider, ScheduledTaskAction, Scheduler, SchedulerHandle, Task, TaskCancellationId,
	TaskInvokeId, TickResult,
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
pub struct TickingSchedulerExecutor<S, C = ()>
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
		}
	}

	pub fn get_current_tick(&mut self) -> Tick {
		self.current_tick
	}

	pub fn tick(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		loop {
			self.drain_scheduler_queue(tick);

			let finished_tasks = self.tick_scheduled_tasks(tick, context);

			self.execute_invoked_tasks(tick, context);

			if finished_tasks.is_empty() {
				break;
			}

			for task_id in finished_tasks.into_iter() {
				self.active_tasks.remove(task_id);
			}
		}
	}

	pub fn tick_by_delta(&mut self, delta: Duration, context: &mut C::Item<'_>) {
		let next_tick = self.get_current_tick() + delta;
		self.tick(next_tick, context);
	}

	fn execute_invoked_tasks(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		for invoked_task_id in self.invoked.drain(..) {
			if let Some(invoked_task) = self.invokable_tasks.get_mut(&invoked_task_id) {
				let invoke_result = invoked_task.tick(tick, context);
				if matches!(invoke_result, TickResult::Done) {
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

	fn tick_scheduled_tasks(&mut self, tick: Tick, context: &mut C::Item<'_>) -> Vec<usize> {
		self.current_tick.update(tick);

		self.active_tasks
			.iter_mut()
			.filter_map(|(key, (_owner_id, task))| {
				let task_result = task.tick(tick, context);
				// TODO: Do something with errors, maybe collect them and return? or define a handler and just pass them into? the tick fn should not have a return type.
				match task_result {
					TickResult::Done => Some(key),
					TickResult::Pending => None,
				}
			})
			.collect::<Vec<_>>()
	}
}
