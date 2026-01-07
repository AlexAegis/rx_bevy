use crate::{ScheduledRepeatedWork, ScheduledWork, WorkContextProvider};

pub trait InvokedTask<Work, TickInput, Context>: ScheduledWork
where
	Work: ScheduledRepeatedWork<TickInput, Context>,
	Context: WorkContextProvider,
{
}

pub trait InvokedTaskFactory<TickInput, Context>
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
