use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{
	ContinuousTaskFactory, ScheduledRepeatedWork, ScheduledWork, WorkContextProvider, WorkResult,
};
use rx_core_macro_work_derive::RxWork;

use crate::Tick;

pub struct TickedContinuousWorkFactory<C>
where
	C: WorkContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> ContinuousTaskFactory<Tick, C> for TickedContinuousWorkFactory<C>
where
	C: 'static + WorkContextProvider,
{
	type Item<Work>
		= TickedContinuousWork<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		TickedContinuousWork {
			last_tick: Tick::default(),
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct TickedContinuousWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Work,
	last_tick: Tick,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> ScheduledWork for TickedContinuousWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> WorkResult {
		if tick_input > self.last_tick {
			self.last_tick.update(tick_input);
			(self.work)(tick_input, context)
		} else {
			WorkResult::Pending
		}
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
