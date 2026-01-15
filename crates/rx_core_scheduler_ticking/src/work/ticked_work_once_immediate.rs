use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{
	ImmediateTaskFactory, PhantomInvariant, ScheduledOnceWork, ScheduledWork, WorkContextProvider,
	WorkResult,
};
use rx_core_macro_work_derive::RxWork;

use crate::Tick;

pub struct TickedImmediateOnceWorkFactory<C>
where
	C: WorkContextProvider,
{
	_phantom_data: PhantomInvariant<C>,
}

impl<C> ImmediateTaskFactory<Tick, C> for TickedImmediateOnceWorkFactory<C>
where
	C: 'static + WorkContextProvider,
{
	type Item<Work>
		= TickedImmediateOnceWork<Work, C>
	where
		Work: ScheduledOnceWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<Tick, C>,
	{
		TickedImmediateOnceWork {
			work: Some(work),
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct TickedImmediateOnceWork<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: WorkContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Option<Work>,
	_phantom_data: PhantomInvariant<C>,
}

impl<Work, C> ScheduledWork for TickedImmediateOnceWork<Work, C>
where
	Work: ScheduledOnceWork<Tick, C>,
	C: WorkContextProvider,
{
	fn tick(&mut self, tick: Tick, context: &mut C::Item<'_>) -> WorkResult {
		if let Some(work) = self.work.take() {
			(work)(tick, context);
		};
		WorkResult::Done
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
