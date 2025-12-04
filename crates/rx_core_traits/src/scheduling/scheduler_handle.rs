use std::sync::{Arc, Mutex};

use disqualified::ShortName;

use crate::{
	Scheduler, SchedulerWithManualTick, Task, TaskCancellationError, TaskId, WithTaskInputOutput,
};

#[derive(Debug)]
pub struct SchedulerHandle<S>
where
	S: Scheduler,
{
	/// Is behind a Mutex because schedulers only have write operations.
	scheduler: Arc<Mutex<S>>,
}

impl<S> Clone for SchedulerHandle<S>
where
	S: Scheduler,
{
	fn clone(&self) -> Self {
		Self {
			scheduler: self.scheduler.clone(),
		}
	}
}

impl<S> SchedulerHandle<S>
where
	S: Scheduler,
{
	pub fn new(scheduler: S) -> Self {
		Self {
			scheduler: Arc::new(Mutex::new(scheduler)),
		}
	}
}

impl<S> From<S> for SchedulerHandle<S>
where
	S: Scheduler,
{
	fn from(scheduler: S) -> Self {
		SchedulerHandle::new(scheduler)
	}
}

impl<S> WithTaskInputOutput for SchedulerHandle<S>
where
	S: Scheduler,
{
	type TickInput = S::TickInput;
	type TaskResult = S::TaskResult;
	type ContextProvider = S::ContextProvider;
	type TaskError = S::TaskError;
}

impl<S> Scheduler for SchedulerHandle<S>
where
	S: Scheduler,
{
	// Since there's a finite type of tasks anyway, it could be an enum
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
		let mut scheduler = self.scheduler.lock().unwrap_or_else(|poison_error| {
			eprintln!("Scheduler ({}) got poisoned!", ShortName::of::<Self>());
			self.scheduler.clear_poison();
			poison_error.into_inner()
		});
		scheduler.schedule(task)
	}

	fn cancel(&mut self, task_id: TaskId) -> Result<(), TaskCancellationError> {
		let mut scheduler = self.scheduler.lock().unwrap_or_else(|poison_error| {
			eprintln!("Scheduler ({}) got poisoned!", ShortName::of::<Self>());
			self.scheduler.clear_poison();
			poison_error.into_inner()
		});

		scheduler.cancel(task_id)
	}
}

impl<S> SchedulerWithManualTick for SchedulerHandle<S>
where
	S: SchedulerWithManualTick,
{
	fn tick(
		&mut self,
		delta_time: std::time::Duration,
		context: &mut <Self::ContextProvider as super::TaskContextProvider>::Item<'_>,
	) {
		let mut scheduler = self.scheduler.lock().unwrap_or_else(|poison_error| {
			eprintln!("Scheduler ({}) got poisoned!", ShortName::of::<Self>());
			self.scheduler.clear_poison();
			poison_error.into_inner()
		});

		scheduler.tick(delta_time, context);
	}
}

pub trait IntoSchedulerHandle<S>
where
	S: Scheduler,
{
	fn into_handle(self) -> SchedulerHandle<S>;
}

impl<S> IntoSchedulerHandle<S> for S
where
	S: Scheduler,
{
	fn into_handle(self) -> SchedulerHandle<S> {
		self.into()
	}
}
