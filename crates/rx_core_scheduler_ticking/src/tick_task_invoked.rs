use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{
	ContextProvider, InvokedTaskFactory, ScheduledRepeatedWork, Task, TaskResult,
};

use crate::Tick;

pub struct InvokedTaskTickedFactory<C>
where
	C: ContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> InvokedTaskFactory<Tick, C> for InvokedTaskTickedFactory<C>
where
	C: 'static + ContextProvider,
{
	type Item<Work>
		= InvokedTaskTicked<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		InvokedTaskTicked {
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct InvokedTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> Task for InvokedTaskTicked<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: ContextProvider,
{
	#[inline]
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> TaskResult {
		(self.work)(tick_input, context)
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
