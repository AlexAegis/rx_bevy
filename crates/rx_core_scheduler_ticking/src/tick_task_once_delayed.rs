use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{ContextProvider, DelayedTask, DelayedTaskFactory, ScheduledOnceWork};

use rx_core_traits::{Task, TaskResult};

use crate::Tick;

pub struct DelayedOnceTaskTickedFactory<C>
where
	C: ContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> DelayedTaskFactory<Tick, C> for DelayedOnceTaskTickedFactory<C>
where
	C: 'static + ContextProvider + Send + Sync,
{
	type Item<Work>
		= DelayedOnceTaskTicked<Work, C>
	where
		Work: ScheduledOnceWork<Tick, C>;
	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<Tick, C>,
	{
		DelayedOnceTaskTicked {
			work: Some(work),
			current_tick: Tick::default(),
			scheduled_on: Tick::default(),
			delay,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct DelayedOnceTaskTicked<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: ContextProvider,
{
	scheduled_on: Tick,
	current_tick: Tick,
	delay: Duration,

	#[derive_where(skip(Debug))]
	work: Option<Work>,

	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> DelayedTask<Work, Tick, C> for DelayedOnceTaskTicked<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: ContextProvider + Send + Sync,
{
}

impl<Work, C> Task for DelayedOnceTaskTicked<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: ContextProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut C::Item<'_>) -> TaskResult {
		self.current_tick.update(tick);
		if self.scheduled_on + self.delay <= self.current_tick {
			let Some(work) = self.work.take() else {
				return TaskResult::Done;
			};

			(work)(tick, context);

			TaskResult::Done
		} else {
			TaskResult::Pending
		}
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::Tick) {
		self.scheduled_on.update(tick_input);
		self.current_tick.update(tick_input);
	}
}
