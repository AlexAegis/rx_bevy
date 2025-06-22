use rx_bevy_observable::{
	Observable, ObservableOutput, Observer, Subscription, UpgradeableObserver,
	prelude::ObserverSubscriber,
};

/// Emits a single value then immediately completes
#[derive(Clone)]
pub struct IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
{
	iterator: Iterator,
}

impl<Iterator, Out> IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
{
	pub fn new(iterator: Iterator) -> Self {
		Self { iterator }
	}
}

impl<Iterator, Out> ObservableOutput for IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
	Out: 'static,
{
	type Out = Out;
	type OutError = ();
}

impl<Iterator, Out> Observable for IteratorObservable<Iterator, Out>
where
	Iterator: Clone + IntoIterator<Item = Out>,
	Out: 'static + Clone,
{
	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
	) -> Subscription {
		let mut subscriber = destination.upgrade();
		for item in self.iterator.clone().into_iter() {
			subscriber.next(item);
		}
		subscriber.complete();
		Subscription::new(subscriber)
	}
}
