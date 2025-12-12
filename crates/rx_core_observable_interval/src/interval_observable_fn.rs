use rx_core_traits::{Scheduler, SchedulerHandle};

use crate::observable::{IntervalObservable, IntervalObservableOptions};

pub fn interval<S>(
	options: IntervalObservableOptions,
	scheduler: SchedulerHandle<S>,
) -> IntervalObservable<S>
where
	S: Scheduler,
{
	IntervalObservable::new(options, scheduler)
}
