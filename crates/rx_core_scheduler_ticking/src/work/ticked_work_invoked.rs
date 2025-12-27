use std::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_work_derive::RxWork;
use rx_core_traits::{
	InvokedTaskFactory, ScheduledRepeatedWork, ScheduledWork, WorkContextProvider, WorkResult,
};

use crate::Tick;

pub struct TickedInvokedWorkFactory<C>
where
	C: WorkContextProvider,
{
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<C> InvokedTaskFactory<Tick, C> for TickedInvokedWorkFactory<C>
where
	C: 'static + WorkContextProvider,
{
	type Item<Work>
		= TickedInvokedWork<Work, C>
	where
		Work: ScheduledRepeatedWork<Tick, C>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<Tick, C>,
	{
		TickedInvokedWork {
			work,
			_phantom_data: PhantomData,
		}
	}
}

#[derive(RxWork)]
#[rx_tick(Tick)]
#[rx_context(C)]
#[derive_where(Debug)]
pub struct TickedInvokedWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	#[derive_where(skip(Debug))]
	work: Work,
	_phantom_data: PhantomData<fn(C) -> C>,
}

impl<Work, C> ScheduledWork for TickedInvokedWork<Work, C>
where
	Work: ScheduledRepeatedWork<Tick, C>,
	C: WorkContextProvider,
{
	#[inline]
	fn tick(&mut self, tick_input: Self::Tick, context: &mut C::Item<'_>) -> WorkResult {
		(self.work)(tick_input, context)
	}

	fn on_scheduled_hook(&mut self, _tick_input: Self::Tick) {}
}
