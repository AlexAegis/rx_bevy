use std::marker::PhantomData;

use rx_bevy_core::{
	Observable, ObservableOutput, SignalContext, Subscriber, SubscriptionHandle, WithContext,
};
use rx_bevy_subscription_inert::InertSubscription;

/// Emits a single value then immediately completes
#[derive(Clone, Debug)]
pub struct IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
{
	iterator: Iterator,
	_phantom_data: PhantomData<*mut Context>,
}

impl<Iterator, Context> IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Context: SignalContext,
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
	Context: SignalContext,
{
	type Out = Iterator::Item;
	type OutError = ();
}

impl<Iterator, Context> WithContext for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
	Context: SignalContext,
{
	type Context = Context;
}

impl<Iterator, Context> Observable for IteratorObservable<Iterator, Context>
where
	Iterator: Clone + IntoIterator,
	Iterator::Item: 'static,
	Context: SignalContext,
{
	type Subscription = InertSubscription<Context>;

	fn subscribe<Destination>(
		&mut self,
		mut destination: Destination,
		context: &mut Self::Context,
	) -> SubscriptionHandle<Self::Subscription>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		for item in self.iterator.clone().into_iter() {
			if destination.is_closed() {
				break;
			}
			destination.next(item, context);
		}

		destination.complete(context);
		SubscriptionHandle::new(InertSubscription::new(destination, context))
	}
}
