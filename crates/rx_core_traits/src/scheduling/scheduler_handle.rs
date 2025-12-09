use std::sync::{Arc, Mutex, MutexGuard};

use disqualified::ShortName;

use crate::{Scheduler, WithContextProvider, WithTaskInputOutput};

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

	pub fn lock(&mut self) -> MutexGuard<'_, S> {
		self.scheduler.lock().unwrap_or_else(|poison_error| {
			eprintln!("Scheduler ({}) got poisoned!", ShortName::of::<Self>());
			self.scheduler.clear_poison();
			poison_error.into_inner()
		})
	}

	pub fn get_scheduler_handle(&self) -> Self {
		self.clone()
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
	type Tick = S::Tick;
}

impl<S> WithContextProvider for SchedulerHandle<S>
where
	S: Scheduler,
{
	type ContextProvider = S::ContextProvider;
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
