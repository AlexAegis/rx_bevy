use rx_core_common::{
	Never, Observable, Observer, Signal, Subscriber, SubscriptionLike, UpgradeableObserver,
};
use rx_core_macro_observable_derive::RxObservable;
use rx_core_subscription_inert::InertSubscription;

/// Emits all values from an iterator then immediately completes.
#[derive(RxObservable, Clone, Debug)]
#[rx_out(Iterator::Item)]
#[rx_out_error(Never)]
pub struct IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	iterator: Iterator,
}

impl<Iterator> IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator> Observable for IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: Signal,
{
	type Subscription<Destination>
		= InertSubscription
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
		for item in self.iterator.clone().into_iter() {
			if destination.is_closed() {
				break;
			}
			destination.next(item);
		}

		if !destination.is_closed() {
			destination.complete();
		}
		InertSubscription::new(destination)
	}
}
