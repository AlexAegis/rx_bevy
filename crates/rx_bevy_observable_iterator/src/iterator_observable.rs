use rx_bevy_core::{
	Observable, ObservableOutput, Observer, Subscription, SubscriptionLike, Teardown,
	UpgradeableObserver,
};

/// Emits a single value then immediately completes
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	iterator: Iterator,
}

impl<Iterator> IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator> ObservableOutput for IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator> Observable for IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut subscriber = destination.upgrade();
		for item in self.iterator.clone().into_iter() {
			if subscriber.is_closed() {
				break;
			}
			subscriber.next(item);
		}
		subscriber.complete();
		Subscription::new(Teardown::Sub(Box::new(subscriber)))
	}
}
