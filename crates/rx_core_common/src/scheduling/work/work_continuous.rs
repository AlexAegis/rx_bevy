use crate::{ScheduledRepeatedWork, ScheduledWork, WorkContextProvider};

pub trait ContinuousTaskFactory<TickInput, Context>
where
	Context: WorkContextProvider,
{
	type Item<Work>: 'static
		+ ScheduledWork<Tick = TickInput, WorkContextProvider = Context>
		+ Send
		+ Sync
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;

	fn new<Work>(work: Work) -> Self::Item<Work>
	where
		Work: ScheduledRepeatedWork<TickInput, Context>;
}

pub trait ContinuousTask<Work, TickInput, Context>: ScheduledWork
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: WorkContextProvider,
{
}
