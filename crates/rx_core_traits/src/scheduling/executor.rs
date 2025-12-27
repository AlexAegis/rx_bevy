use crate::{Scheduler, SchedulerHandle, WithWorkContextProvider, WithWorkInputOutput};

pub trait WorkExecutor: WithWorkInputOutput + WithWorkContextProvider {
	type Scheduler: Scheduler<Tick = Self::Tick, WorkContextProvider = Self::WorkContextProvider>;

	fn get_scheduler_handle(&self) -> SchedulerHandle<Self::Scheduler>;
}
