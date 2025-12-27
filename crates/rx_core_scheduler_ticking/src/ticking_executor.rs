use std::{
	collections::{HashMap, HashSet},
	time::Duration,
};

use derive_where::derive_where;
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_traits::{
	ScheduledWork, ScheduledWorkAction, Scheduler, SchedulerHandle, WorkCancellationId,
	WorkContextProvider, WorkInvokeId, WorkResult,
};
use slab::Slab;

use crate::Tick;

pub trait SchedulerForTickingExecutor: Scheduler {
	fn update_tick(&mut self, tick: Tick);

	fn drain_actions(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledWorkAction<Tick, Self::WorkContextProvider>>;

	/// Returns true if there are work actions queued.
	fn has_actions(&self) -> bool;
}

#[derive_where(Default; S)]
#[derive(RxExecutor)]
#[rx_context(C)]
#[rx_tick(Tick)]
#[rx_scheduler(S)]
pub struct TickingSchedulerExecutor<S, C>
where
	S: Scheduler<Tick = Tick, WorkContextProvider = C>,
	C: 'static + WorkContextProvider + Send + Sync,
{
	#[scheduler_handle]
	scheduler: SchedulerHandle<S>,

	current_tick: Tick,

	// TODO: ADD FROM THE BEVY SIDE SOME NEW TASK TYPES LIKE SEND EVENT, THAT NEEDS NO CALLBACKS
	active_work: Slab<(
		WorkCancellationId,
		Box<dyn ScheduledWork<Tick = Tick, WorkContextProvider = C> + Send + Sync>,
	)>,
	invokable_work: HashMap<
		WorkInvokeId,
		Box<dyn ScheduledWork<Tick = Tick, WorkContextProvider = C> + Send + Sync>,
	>,
	invoked: Vec<WorkInvokeId>,
	already_ticked: HashSet<usize>,
}

impl<S, C> TickingSchedulerExecutor<S, C>
where
	S: SchedulerForTickingExecutor<Tick = Tick, WorkContextProvider = C>,
	C: 'static + WorkContextProvider + Send + Sync,
{
	pub fn new(scheduler: S) -> Self {
		Self {
			current_tick: Tick::default(),
			active_work: Slab::new(),
			invokable_work: HashMap::new(),
			invoked: Vec::new(),
			scheduler: SchedulerHandle::new(scheduler),
			already_ticked: HashSet::new(),
		}
	}

	pub fn now(&self) -> Duration {
		self.current_tick.elapsed_since_start
	}

	/// Returns `true` when there is no active work in the executor.
	pub fn is_empty(&self) -> bool {
		self.active_work.is_empty()
	}

	pub fn get_current_tick(&self) -> Tick {
		self.current_tick
	}

	pub fn tick_to(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		self.current_tick.update(tick);
		self.already_ticked.clear();
		loop {
			self.drain_scheduler_queue(tick);

			let no_work_ticked = self.tick_scheduled(tick, context);

			self.execute_invoked(tick, context);

			if no_work_ticked {
				break;
			}
		}
	}

	pub fn tick(&mut self, delta: Duration, context: &mut C::Item<'_>) {
		let next_tick = self.get_current_tick() + delta;
		self.tick_to(next_tick, context);
	}

	fn execute_invoked(&mut self, tick: Tick, context: &mut C::Item<'_>) {
		for invoked_id in self.invoked.drain(..) {
			if let Some(invoked_work) = self.invokable_work.get_mut(&invoked_id) {
				let invoke_result = invoked_work.tick(tick, context);
				if matches!(invoke_result, WorkResult::Done) {
					self.invokable_work.remove(&invoked_id);
				}
			}
		}
	}

	fn drain_scheduler_queue(&mut self, tick: Tick) {
		let mut scheduler = self.scheduler.lock();
		scheduler.update_tick(tick);

		for action in scheduler.drain_actions() {
			match action {
				ScheduledWorkAction::<Tick, C>::Activate((owner_id, work)) => {
					self.active_work.insert((owner_id, work));
				}
				ScheduledWorkAction::AddInvoked((invoke_id, work)) => {
					self.invokable_work.insert(invoke_id, work);
				}
				ScheduledWorkAction::Invoke(invoke_id) => self.invoked.push(invoke_id),
				ScheduledWorkAction::CancelInvoked(cancelled_invokation_id) => {
					self.invokable_work.remove(&cancelled_invokation_id);
					self.invoked
						.retain(|invoked_id| invoked_id == &cancelled_invokation_id);
				}
				ScheduledWorkAction::Cancel(cancelled_id) => {
					self.active_work
						.retain(|_work_id, (work_cancellation_id, _work)| {
							work_cancellation_id != &cancelled_id
						});
				}
			}
		}
	}

	fn tick_scheduled(&mut self, tick: Tick, context: &mut C::Item<'_>) -> bool {
		let mut work_done = Vec::<usize>::new();
		let mut work_ticked = Vec::<usize>::new();

		for (key, (_, work)) in self
			.active_work
			.iter_mut()
			.filter(|(key, _)| !self.already_ticked.contains(key))
		{
			let work_result = work.tick(tick, context);

			if matches!(work_result, WorkResult::Done) {
				work_done.push(key);
			}

			work_ticked.push(key);
		}

		for key in work_done {
			self.active_work.remove(key);
		}

		let no_work_ticked = work_ticked.is_empty();

		for key in work_ticked {
			self.already_ticked.insert(key);
		}

		no_work_ticked
	}
}
