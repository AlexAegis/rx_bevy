use core::{num::NonZero, time::Duration};

use crate::{ScheduledWork, WorkContextProvider, WorkResult};

pub trait RepeatedTaskFactory<TickInput, Context>
where
	Context: WorkContextProvider,
{
	type Item<Work>: 'static
		+ ScheduledWork<Tick = TickInput, WorkContextProvider = Context>
		+ Send
		+ Sync
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;

	fn new<Work>(
		work: Work,
		interval: Duration,
		start_immediately: bool,
		max_work_per_tick: NonZero<usize>,
	) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;
}

pub trait RepeatedTask<Work, TickInput, Context>: ScheduledWork
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: WorkContextProvider,
{
}

pub trait ScheduledRepeatedWork<TickInput, Context>:
	'static + FnMut(TickInput, &mut Context::Item<'_>) -> WorkResult + Send + Sync
where
	Context: WorkContextProvider,
{
}

impl<W, TickInput, Context> ScheduledRepeatedWork<TickInput, Context> for W
where
	Context: WorkContextProvider,
	W: 'static + FnMut(TickInput, &mut Context::Item<'_>) -> WorkResult + Send + Sync,
{
}
