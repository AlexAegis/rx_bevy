use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Never, Observable, Observer, Scheduler, SchedulerHandle, Subscriber, UpgradeableObserver,
};

use crate::{IntervalSubscription, observable::IntervalObservableOptions};

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
