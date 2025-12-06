use std::sync::{Arc, Mutex, MutexGuard};

use disqualified::ShortName;

use crate::{Scheduler, WithTaskInputOutput};

#[derive(Debug, Default)]
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

	pub fn get_scheduler(&mut self) -> MutexGuard<'_, S> {
		self.scheduler.lock().unwrap_or_else(|poison_error| {
			eprintln!("Scheduler ({}) got poisoned!", ShortName::of::<Self>());
			self.scheduler.clear_poison();
			poison_error.into_inner()
		})
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
	type ContextProvider = S::ContextProvider;
	type TaskError = S::TaskError;
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
