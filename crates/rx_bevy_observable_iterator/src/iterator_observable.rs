use std::marker::PhantomData;

use rx_bevy_core::{DropContext, DropSafeSignalContext};
use rx_bevy_core::{Observable, ObservableOutput, SignalContext, Subscriber};
use rx_bevy_subscription_drop::DropSubscription;

/// Emits a single value then immediately completes
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
{
	iterator: Iterator,
	_phantom_data: PhantomData<Context>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
{
	pub fn new(iterator: Iterator) -> Self {
		Self {
			iterator,
			_phantom_data: PhantomData,
		}
	}
}

impl<Iterator, Context> ObservableOutput for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> SignalContext for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<Iterator, Context> Observable for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Subscription = DropSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
		context: &mut <Destination as SignalContext>::Context,
	) -> Self::Subscription
	where
		Destination: 'static
			+ Subscriber<
				In = Self::Out,
				InError = Self::OutError,
				Context = <Self::Subscription as SignalContext>::Context,
			>,
	{
		let mut subscriber = destination;
		for item in self.iterator.clone().into_iter() {
			if subscriber.is_closed() {
				break;
			}
			subscriber.next(item, context);
		}
		subscriber.complete(context);
		DropSubscription::new(subscriber)
	}
}
