use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Never, Observable, Observer, Scheduler, SchedulerHandle, Subscriber, UpgradeableObserver,
};

use crate::{IntervalSubscription, observable::IntervalObservableOptions};

/// # IntervalObservable
///
/// Emits sequential numbers every time the interval is elapsed in the provided
/// scheduler.
///
/// ## Completion Behavior
///
/// This observable does **NOT** complete. It emits numbers infinitely, until
/// unsubscribed.
///
/// ## Error Behavior
///
/// This observable does not error.
///
/// ## Arguments
///
/// - `options`: Configuration of behavior
///   - `duration`: How much time must elapse between emissions
///
///     Default: 1 sec
///   - `start_on_subscribe`: Whether or not the first emission, `0` should
///     happen on subscribe or after the duration had elapsed once.
///     Note that when this is `true`, the first emission happens "on subscribe"
///     meaning, immediately, and outside of the scheduler!
///
///     Default: false
///   - `max_emissions_per_tick`:
///     If the internal timer rolls over multiple times during a single tick,
///     all of them will result in an emission. To prevent emitting too much
///     during a particularly large tick, for example during a lagged frame,
///     this limit ensures at most this many emissions can happen.
///
///     This setting is mostly relevant for short intervals, shorter than
///     a slow framerate. (Ignore the stable, high framerate you're targeting,
///     what matters is what happens when it slows down!)
///     
///     - When downstream is expensive and it's okay to lose some emissions:
///       You may want to set this to 1 if you don't want to make a lag-spike
///       even worse, when reacting to the interval rolling over multiple times
///       would perform something expensive multiple times.
///     - When time-keeping is important and downstream computation is cheap:
///       You can safely keep this at a higher value to not lose any time
///       measured.
///
///     Since setting this to `0` would result in the interval not doing
///     anything, 1 will be used instead.
///    
///     Default: 10
/// - `scheduler`: The scheduler's handle to drive the interval.
///   Typically sourced from an executor which can differ from environment to
///   environment.
#[derive(RxObservable, Debug)]
#[rx_out(usize)]
#[rx_out_error(Never)]
pub struct IntervalObservable<S>
where
	S: Scheduler,
{
	options: IntervalObservableOptions,
	scheduler: SchedulerHandle<S>,
}

impl<S> IntervalObservable<S>
where
	S: Scheduler,
{
	pub fn new(options: IntervalObservableOptions, scheduler: SchedulerHandle<S>) -> Self {
		Self { options, scheduler }
	}
}

impl<S> Observable for IntervalObservable<S>
where
	S: 'static + Scheduler + Send + Sync,
{
	type Subscription<Destination>
		= IntervalSubscription<Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		observer: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	{
		let mut destination = observer.upgrade();
		if self.options.start_on_subscribe {
			destination.next(0);
		}
		IntervalSubscription::new(destination, self.options.clone(), self.scheduler.clone())
	}
}
