use std::time::Duration;

use crate::{ScheduledWork, WorkContextProvider};

pub trait DelayedWork<Work, TickInput, Context>: ScheduledWork
where
	Work: ScheduledOnceWork<TickInput, Context>,
	Context: WorkContextProvider,
{
}

pub trait DelayedWorkFactory<TickInput, Context>
where
	Context: WorkContextProvider,
{
	type Item<Work>: 'static
		+ ScheduledWork<Tick = TickInput, WorkContextProvider = Context>
		+ Send
		+ Sync
	where
		Work: ScheduledOnceWork<TickInput, Context>;

	fn new<Work>(work: Work, delay: Duration) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, Context>;
}

pub trait ScheduledOnceWork<TickInput, Context>:
	'static + FnOnce(TickInput, &mut Context::Item<'_>) + Send + Sync
where
	Context: WorkContextProvider,
{
}

impl<W, TickInput, Context> ScheduledOnceWork<TickInput, Context> for W
where
	Context: WorkContextProvider,
	W: 'static + FnOnce(TickInput, &mut Context::Item<'_>) + Send + Sync,
{
}
