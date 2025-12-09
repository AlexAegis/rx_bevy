use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{
	ContextProvider, RepeatedTaskFactory, ScheduledRepeatedWork, Task, TickResult,
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

	fn new<Work>(work: Work, interval: Duration, start_immediately: bool) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
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
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> Task for DelayedRepeatingTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> TickResult {
		self.current_tick.update(tick_input);

		let mut tick_result = TickResult::Pending;
		while self.consumed_until + self.interval <= self.current_tick {
			self.consumed_until += self.interval;
			tick_result += (self.work)(tick_input, context);
		}
		tick_result
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
