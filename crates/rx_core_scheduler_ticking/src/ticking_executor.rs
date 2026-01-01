use std::{
	collections::{HashMap, HashSet},
	time::Duration,
};

use indexmap::IndexMap;
use rx_core_macro_executor_derive::RxExecutor;
use rx_core_traits::{
	ScheduledWork, ScheduledWorkAction, Scheduler, SchedulerHandle, WorkCancellationId,
	WorkContextProvider, WorkInvokeId, WorkResult,
};

use crate::{Tick, WorkId, WorkIdGenerator};

pub trait SchedulerForTickingExecutor: Scheduler {
	fn update_tick(&mut self, tick: Tick);

	fn drain_actions(
		&mut self,
	) -> std::vec::Drain<'_, ScheduledWorkAction<Tick, Self::WorkContextProvider>>;

	/// Returns true if there are work actions queued.
	fn has_actions(&self) -> bool;
}

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
	work_id_generator: WorkIdGenerator,
	// TODO: ADD FROM THE BEVY SIDE SOME NEW TASK TYPES LIKE SEND EVENT, THAT NEEDS NO CALLBACKS
	active_work: IndexMap<
		WorkId,
		Box<dyn ScheduledWork<Tick = Tick, WorkContextProvider = C> + Send + Sync>,
	>,
	// TODO: Use ManyMap once done to make sure CancellationIds can't grow infinetly, they probably don't already as subscriptions always use the cancellationId, but I want to make it sure.
	cancellation_map: HashMap<WorkCancellationId, Vec<WorkId>>,
	invokable_work: HashMap<
		WorkInvokeId,
		Box<dyn ScheduledWork<Tick = Tick, WorkContextProvider = C> + Send + Sync>,
	>,
	invoked: Vec<WorkInvokeId>,
	already_ticked: HashSet<WorkId>,
	max_single_tick_recursion_depth: usize,
}

impl<S, C> TickingSchedulerExecutor<S, C>
where
	S: SchedulerForTickingExecutor<Tick = Tick, WorkContextProvider = C>,
	C: 'static + WorkContextProvider + Send + Sync,
{
	pub fn new(scheduler: S) -> Self {
		Self {
			current_tick: Tick::default(),
			active_work: IndexMap::default(),
			work_id_generator: WorkIdGenerator::default(),
			cancellation_map: HashMap::default(),
			invokable_work: HashMap::new(),
			invoked: Vec::new(),
			scheduler: SchedulerHandle::new(scheduler),
			already_ticked: HashSet::new(),
			max_single_tick_recursion_depth: 100,
		}
	}

	/// Used mainly for testing! (To be able to reach the limit sooner)
	///
	/// Default value: 100
	///
	/// Definitely do not lower this below 10.
	///
	/// This depth can be reached by recursively scheduling work
	/// that is then immediately scheduling work that can also be immediately
	/// executed and so on.
	///
	/// > Nested delayed work above a delay of 0 is safe, and can not reach
	/// > this limit ever! Such work scheduled from a scheudled task can only
	/// > start from the next tick, as the "scheduled_on" time of the next
	/// > task is the already elapsed time of the current task. And the delay
	/// > accounts for that time too.
	pub fn with_max_single_tick_recursion_depth(
		self,
		max_single_tick_recursion_depth: usize,
	) -> Self {
		Self {
			max_single_tick_recursion_depth,
			..self
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

		// It should almost always finish in 1-2 loops, but in case something
		// causes an infinitely scheduling recursive work, it should be loud.
		for i in 0..=self.max_single_tick_recursion_depth {
			let had_actions = self.drain_scheduler_queue(tick);

			let no_work_ticked = self.tick_scheduled(tick, context);

			self.execute_invoked(tick, context);

			if no_work_ticked && !had_actions {
				break;
			}

			if i == self.max_single_tick_recursion_depth {
				panic!(
					"The executor encountered recursive work {} layers deep!",
					self.max_single_tick_recursion_depth
				)
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

	fn drain_scheduler_queue(&mut self, tick: Tick) -> bool {
		let mut scheduler = self.scheduler.lock();
		let had_actions = scheduler.has_actions();
		scheduler.update_tick(tick);

		for action in scheduler.drain_actions() {
			match action {
				ScheduledWorkAction::<Tick, C>::Activate((cancellation_id, work)) => {
					let work_id = self.work_id_generator.get_next();
					self.active_work.insert(work_id, work);
					self.cancellation_map
						.entry(cancellation_id)
						.or_default()
						.push(work_id);
				}
				ScheduledWorkAction::AddInvoked((invoke_id, work)) => {
					self.invokable_work.insert(invoke_id, work);
				}
				ScheduledWorkAction::Invoke(invoke_id) => {
					self.invoked.push(invoke_id);
				}
				ScheduledWorkAction::CancelInvoked(cancelled_invokation_id) => {
					self.invokable_work.remove(&cancelled_invokation_id);
					self.invoked
						.retain(|invoked_id| invoked_id == &cancelled_invokation_id);
				}
				ScheduledWorkAction::Cancel(cancelled_id) => {
					if let Some(work_ids) = self.cancellation_map.remove(&cancelled_id) {
						for work_id in work_ids {
							self.active_work.shift_remove(&work_id);
						}
					}
				}
			}
		}

		had_actions
	}

	fn tick_scheduled(&mut self, tick: Tick, context: &mut C::Item<'_>) -> bool {
		// Each work gets to tick once per tick. This is tracked so that if
		// new work is scheduled, that also gets ticked in a new loop
		// without ticking ones already ticked this loop
		let mut work_ticked = Vec::<WorkId>::new();
		let mut work_finished_this_tick = Vec::<WorkId>::new();

		for (work_id, work) in self
			.active_work
			.iter_mut()
			.filter(|(key, _)| !self.already_ticked.contains(key))
		{
			let work_result = work.tick(tick, context);

			if matches!(work_result, WorkResult::Done) {
				work_finished_this_tick.push(*work_id);
			}

			work_ticked.push(*work_id);
		}

		let no_work_ticked = work_ticked.is_empty();

		for key in work_ticked {
			self.already_ticked.insert(key);
		}

		for work_id in work_finished_this_tick {
			self.active_work.shift_remove(&work_id);
		}

		no_work_ticked
	}
}
