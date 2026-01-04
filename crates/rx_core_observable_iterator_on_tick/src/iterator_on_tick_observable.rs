use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{
	Never, Observable, Observer, Scheduler, SchedulerHandle, Signal, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};

use crate::{OnTickIteratorSubscription, observable::OnTickObservableOptions};

/// Emits an iterators values one at a time at every nth tick, regardless how
/// long each tick was. Mostly meant for debugging purposes, or just to observe
/// `n` amount of steady ticks of the scheduler used.
///
/// > Warning! This is not the same thing as creating a timer, for that use
/// > the [rx_core_observable_interval::IntervalObservable]!
///
/// An example usecase is throttling a logger to every nth frame, where knowing
/// exactly how many frames have passed is useful. Otherwise, the
/// IntervalObservable is a better choice for throttling.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Iterator::Item)]
#[rx_out_error(Never)]
pub struct IteratorOnTickObservable<Iterator, S>
where
	Iterator: 'static + Clone + IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	iterator: Iterator,
	options: OnTickObservableOptions,
	scheduler: SchedulerHandle<S>,
}

impl<Iterator, S> IteratorOnTickObservable<Iterator, S>
where
	Iterator: 'static + Clone + IntoIterator,
	Iterator::Item: Signal,
	S: Scheduler,
{
	pub fn new(
		iterator: Iterator,
		options: OnTickObservableOptions,
		scheduler: SchedulerHandle<S>,
	) -> Self {
		Self {
			iterator,
			options,
			scheduler,
		}
	}
}

impl<Iterator, S> Observable for IteratorOnTickObservable<Iterator, S>
where
	Iterator: 'static + Clone + IntoIterator,
	Iterator::Item: Signal,
	Iterator::IntoIter: Send + Sync,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscription<Destination>
		= OnTickIteratorSubscription<Destination, Iterator, S>
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
		let mut iter = self.iterator.clone().into_iter();
		if self.options.emit_at_every_nth_tick == 0 {
			let mut completed = true;
			for item in iter.by_ref() {
				if destination.is_closed() {
					completed = false;
					break;
				}
				destination.next(item);
			}
			if completed && !destination.is_closed() {
				destination.complete();
			}
		}
		OnTickIteratorSubscription::new(
			destination,
			iter,
			self.options.clone(),
			self.scheduler.clone(),
		)
	}
}
