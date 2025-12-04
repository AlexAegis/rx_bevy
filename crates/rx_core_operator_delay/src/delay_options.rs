use std::time::Duration;

use derive_where::derive_where;
use rx_core_traits::{Scheduler, SchedulerHandle};

#[derive(Debug)]
#[derive_where(Clone)]
pub struct DelayOperatorOptions<S>
where
	S: Scheduler,
{
	/// How much to delay each upstream emission before re-emitted downstream?
	pub delay: Duration,

	pub scheduler: SchedulerHandle<S>,
}
