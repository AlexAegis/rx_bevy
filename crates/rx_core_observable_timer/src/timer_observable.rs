use std::time::Duration;

use rx_core_common::{
	Never, Observable, Scheduler, SchedulerHandle, Subscriber, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;

use crate::TimerSubscription;

/// # TimerObservable
///
/// Emits a single `()` after a duration, then completes.
#[derive(RxObservable, Debug)]
#[rx_out(())]
#[rx_out_error(Never)]
pub struct TimerObservable<S>
where
	S: Scheduler,
{
	duration: Duration,
	scheduler: SchedulerHandle<S>,
}

impl<S> TimerObservable<S>
where
	S: Scheduler,
{
	pub fn new(duration: Duration, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			duration,
			scheduler,
		}
	}
}

impl<S> Observable for TimerObservable<S>
where
	S: 'static + Scheduler + Send + Sync,
{
	type Subscription<Destination>
		= TimerSubscription<Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		TimerSubscription::new(destination.upgrade(), self.duration, self.scheduler.clone())
	}
}
