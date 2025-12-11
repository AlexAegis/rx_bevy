use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{
	ContextProvider, RepeatedTaskFactory, ScheduledRepeatedWork, Task, TaskResult,
};

use crate::Tick;

pub struct RepeatedTaskTickedFactory<C>
where
	C: ContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> RepeatedTaskFactory<Tick, C> for RepeatedTaskTickedFactory<C>
where
	C: 'static + ContextProvider,
{
	type Item<Work>
		= DelayedRepeatingTaskTicked<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(
		work: Work,
		interval: Duration,
		start_immediately: bool,
		max_work_per_tick: usize,
	) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		DelayedRepeatingTaskTicked {
			start_immediately,
			consumed_until: Tick::default(),
			current_tick: Tick::default(),
			interval,
			max_work_per_tick,
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct DelayedRepeatingTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	/// The work will be executed on the first tick too, regardless if the timer
	/// had elapsed or not.
	start_immediately: bool,
	consumed_until: Tick,
	current_tick: Tick,
	interval: Duration,
	max_work_per_tick: usize,
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> Task for DelayedRepeatingTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> TaskResult {
		self.current_tick.update(tick_input);

		let mut task_result = TaskResult::Pending;
		let mut executions = 0;
		while self.consumed_until + self.interval <= self.current_tick
			&& !matches!(task_result, TaskResult::Done)
		{
			if executions < self.max_work_per_tick {
				task_result += (self.work)(tick_input, context);
			}
			// The consumed until marker has to advance all the way,
			// regardless of how much work was allowed to execute
			self.consumed_until += self.interval;
			executions += 1;
		}
		task_result
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::Tick) {
		if self.start_immediately {
			self.consumed_until.update(tick_input - self.interval);
		} else {
			self.consumed_until.update(tick_input);
		}

		self.current_tick.update(tick_input);
	}
}
