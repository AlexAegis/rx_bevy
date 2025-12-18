use std::time::Duration;

use rx_core_traits::{Scheduler, SchedulerHandle};

use crate::observable::TimerObservable;

pub fn timer<S>(duration: Duration, scheduler: SchedulerHandle<S>) -> TimerObservable<S>
where
	S: Scheduler,
{
	TimerObservable::new(duration, scheduler)
}
