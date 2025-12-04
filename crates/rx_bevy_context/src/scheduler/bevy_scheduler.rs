use std::marker::PhantomData;

use rx_bevy_common::Clock;
use rx_core_scheduler_ticking::TickingScheduler;
use rx_core_traits::{Scheduler, Task, TaskId, Tick, WithTaskInputOutput};

use crate::RxBevyContext;

pub struct BevyRxScheduler<C>
where
	C: Clock,
{
	ticking_scheduler: TickingScheduler<RxBevyContext>,
	_phantom_data: PhantomData<C>,
}

impl<C> WithTaskInputOutput for BevyRxScheduler<C>
where
	C: Clock,
{
	type TickInput = Tick;
	type TaskResult = ();
	type TaskError = ();
	type ContextProvider = RxBevyContext;
}

impl<C> Scheduler for BevyRxScheduler<C>
where
	C: Clock,
{
	fn schedule<T>(&mut self, task: T) -> TaskId
	where
		T: 'static
			+ Task<
				TickInput = Self::TickInput,
				TaskResult = Self::TaskResult,
				TaskError = Self::TaskError,
				ContextProvider = Self::ContextProvider,
			>,
	{
		self.ticking_scheduler.schedule(task)
	}

	fn cancel(&mut self, task_id: TaskId) -> Result<(), rx_core_traits::TaskCancellationError> {
		self.ticking_scheduler.cancel(task_id)
	}
}
