use rx_bevy_core::{
	ChannelContext, DropContext, DropSubscription, Observable, ObservableOutput, Observer,
	SubscriptionLike, Teardown, UpgradeableObserver,
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

impl<Iterator, Context> Observable for IteratorObservable<Iterator>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
	Context: DropContext,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<
		Destination: 'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError>,
	>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as Observer>::Context,
	) -> DropSubscription<Context> {
		let mut subscriber = destination.upgrade();
		for item in self.iterator.clone().into_iter() {
			if subscriber.is_closed() {
				break;
			}
			subscriber.next(item, context);
		}
		subscriber.complete(context);
		DropSubscription::new(Teardown::Sub(Box::new(subscriber)))
	}
}
