use core::{marker::PhantomData, num::NonZero, time::Duration};

use derive_where::derive_where;
use rx_core_macro_work_derive::RxWork;
use rx_core_traits::{
	RepeatedTaskFactory, ScheduledRepeatedWork, ScheduledWork, WorkContextProvider, WorkResult,
};

use crate::Tick;

pub struct TickedRepeatingWorkFactory<C>
where
	C: WorkContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> RepeatedTaskFactory<Tick, C> for TickedRepeatingWorkFactory<C>
where
	C: 'static + WorkContextProvider,
{
	type Item<Work>
		= TickedRepeatingWork<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(
		work: Work,
		interval: Duration,
		start_immediately: bool,
		max_work_per_tick: NonZero<usize>,
	) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		TickedRepeatingWork {
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

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct TickedRepeatingWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	/// The work will be executed on the first tick too, regardless if the timer
	/// had elapsed or not.
	start_immediately: bool,
	consumed_until: Tick,
	current_tick: Tick,
	interval: Duration,
	max_work_per_tick: NonZero<usize>,
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> ScheduledWork for TickedRepeatingWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> WorkResult {
		self.current_tick.update(tick_input);

		let mut work_result = WorkResult::Pending;
		let mut executions: usize = 0;

		while (self.consumed_until + self.interval <= self.current_tick
			&& !matches!(work_result, WorkResult::Done))
			|| self.start_immediately
		{
			if executions < self.max_work_per_tick.into() {
				work_result += (self.work)(tick_input, context);
			}
			// The consumed until marker has to advance all the way,
			// regardless of how much work was allowed to execute
			if !self.start_immediately {
				self.consumed_until += self.interval;
			} else {
				self.start_immediately = false;
			}
			executions += 1;
		}
		work_result
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::Tick) {
		self.consumed_until.update(tick_input);
		self.current_tick.update(tick_input);
	}
}
