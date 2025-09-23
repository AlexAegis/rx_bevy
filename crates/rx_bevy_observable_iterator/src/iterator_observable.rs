use std::marker::PhantomData;

use rx_bevy_core::{DropContext, DropSafeSignalContext};
use rx_bevy_core::{Observable, ObservableOutput, SignalContext, Subscriber};
use rx_bevy_subscription_drop::DropSubscription;

/// Emits a single value then immediately completes
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	iterator: Iterator,
	_phantom_data: PhantomData<Context>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: DropContext<DropSafety = DropSafeSignalContext>,
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
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Out = Iterator::Item;
	type OutError = ();
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
		mut destination: Destination,
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
		for item in self.iterator.clone().into_iter() {
			if destination.is_closed() {
				break;
			}
			destination.next(item, context);
		}
		destination.complete(context);

		DropSubscription::new(destination, false)
	}
}
