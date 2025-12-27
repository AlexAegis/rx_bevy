use crate::{ScheduledOnceWork, ScheduledWork, WorkContextProvider};

pub trait ImmediateTask<Work, TickInput, Context>: ScheduledWork
where
	Work: ScheduledOnceWork<TickInput, Context>,
	Context: WorkContextProvider,
{
}

pub trait ImmediateTaskFactory<TickInput, Context>
where
	Context: WorkContextProvider,
{
	type Item<Work>: 'static
		+ ScheduledWork<Tick = TickInput, WorkContextProvider = Context>
		+ Send
		+ Sync
	where
		Work: ScheduledOnceWork<TickInput, Context>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledOnceWork<TickInput, Context>;
}
