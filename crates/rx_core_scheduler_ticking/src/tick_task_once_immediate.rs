use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_task_derive::RxTask;
use rx_core_traits::{ContextProvider, ImmediateTaskFactory, ScheduledOnceWork, Task, TaskResult};

use crate::Tick;

pub struct ImmediateOnceTaskTickedFactory<C>
where
	C: ContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> ImmediateTaskFactory<Tick, C> for ImmediateOnceTaskTickedFactory<C>
where
	C: 'static + ContextProvider,
{
	type Item<Work>
		= ImmediateOnceTaskTicked<Work, C>
	where
		Work: ScheduledOnceWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<Tick, C>,
	{
		ImmediateOnceTaskTicked {
			work: Some(work),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxTask)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct ImmediateOnceTaskTicked<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: ContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Option<Work>,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> Task for ImmediateOnceTaskTicked<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: ContextProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut C::Item<'_>) -> TaskResult {
		let Some(work) = self.work.take() else {
			return TaskResult::Done;
		};
		(work)(tick, context);
		TaskResult::Done
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
