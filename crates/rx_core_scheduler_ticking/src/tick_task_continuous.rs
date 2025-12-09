use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{
	ContextProvider, ContinuousTaskFactory, ScheduledRepeatedWork, Task, TickResult,
};

use crate::Tick;

pub struct ContinuousTaskTickedFactory<C>
where
	C: ContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> ContinuousTaskFactory<Tick, C> for ContinuousTaskTickedFactory<C>
where
	C: 'static + ContextProvider,
{
	type Item<Work>
		= ContinuousTaskTicked<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		ContinuousTaskTicked {
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct ContinuousTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> Task for ContinuousTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> TickResult {
		(self.work)(tick_input, context)
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
