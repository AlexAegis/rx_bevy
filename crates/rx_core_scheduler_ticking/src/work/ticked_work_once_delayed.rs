use std::{marker::PhantomData, time::Duration};

use derive_where::derive_where;
use rx_core_macro_work_derive::RxWork;
use rx_core_traits::{DelayedWork, DelayedWorkFactory, ScheduledOnceWork, WorkContextProvider};

use rx_core_traits::{ScheduledWork, WorkResult};

use crate::Tick;

pub struct TickedDelayedOnceWorkFactory<C>
where
	C: WorkContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> DelayedWorkFactory<Tick, C> for TickedDelayedOnceWorkFactory<C>
where
	C: 'static + WorkContextProvider + Send + Sync,
{
	type Item<Work>
		= TickedDelayedOnceWork<Work, C>
	where
		Work: ScheduledOnceWork<Tick, C>;
	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<Tick, C>,
	{
		TickedDelayedOnceWork {
			work: Some(work),
			current_tick: Tick::default(),
			scheduled_on: Tick::default(),
			delay,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct TickedDelayedOnceWork<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: WorkContextProvider,
{
	scheduled_on: Tick,
	current_tick: Tick,
	delay: Duration,

	#[derive_where(skip(Debug))]
	work: Option<Work>,

	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> DelayedWork<Work, Tick, C> for TickedDelayedOnceWork<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: WorkContextProvider + Send + Sync,
{
}

impl<Work, C> ScheduledWork for TickedDelayedOnceWork<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: WorkContextProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut C::Item<'_>) -> WorkResult {
		self.current_tick.update(tick);
		if self.scheduled_on + self.delay <= self.current_tick {
			if let Some(work) = self.work.take() {
				(work)(tick, context);
			}
			WorkResult::Done
		} else {
			WorkResult::Pending
		}
	}

	fn on_scheduled_hook(&mut self, tick_input: Self::Tick) {
		self.scheduled_on.update(tick_input);
		self.current_tick.update(tick_input);
	}
}
